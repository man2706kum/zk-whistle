declare var self: Worker;

import { ProofLeakVerifier } from '../utils/plVerifier';
import os from 'os'

const proofLeakBB = new ProofLeakVerifier("honk", os.cpus.length)

self.onmessage = async (event: MessageEvent) => {
  console.time(`proof: ${event.data.uuid} `)
  const result = await proofLeakBB.verify(event.data)
  console.timeEnd(`proof: ${event.data.uuid} `)
  postMessage({ uuid : event.data.uuid, result});
};