import { v4 as generateUuid } from 'uuid'
import { VerificationModel } from '../models/Verification';
import { keccak256 } from 'ethers'
import type { Context } from 'hono'

async function proofExists(proofHash: string): Promise<string | null> {
    const result = await VerificationModel.findOne({ proof_hash: proofHash }).exec();
    return result == null ? null : result.uuid;
}

export async function createVerification(c: Context) {
    //@ts-ignore
    const { proofBase64, publicInputs } = await c.req.json();
    const proof = Buffer.from(proofBase64, 'base64')
    const proofHash = keccak256(proof).slice(2)

    let uuid = await proofExists(proofHash);
    if (uuid != null) {
        return Response.json({ success: false, error: "Proof already exists", id: uuid }, {
            status: 400
        })
    } 
    
    uuid = generateUuid();
    const verification = new VerificationModel({ proof_hash: proofHash, uuid });
    const savedVerification = await verification.save();
    const worker = new Worker(new URL("../utils/worker.ts", import.meta.url));
    worker.onmessage = async event => {
        await VerificationModel.findOne({ uuid: event.data.uuid }).updateOne({ status: event.data.result ? 'valid' : 'invalid' });
        console.log(`Status updated for ${event.data.uuid} , ${event.data.result}  `)
        worker.terminate();
    };
    worker.postMessage({ proof, publicInputs, uuid });
    c.status(201)
    return c.json({ success: true, id: savedVerification.uuid, });
}

export async function verificationStatus(c: Context) {
    const { id } = c.req.param();

    try {
        //@ts-ignore
        const { status } = await VerificationModel.findOne({ uuid: id }).exec()
        return c.json({ status });
    } catch (err) {
        c.status(500)
        return c.json({ error: "Unable to process request" })
    }

}