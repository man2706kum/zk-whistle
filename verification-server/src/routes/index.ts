import { createVerification,verificationStatus } from "../controllers/verifyController";

export const routes   =
{
    "/api/verify": {
        POST: createVerification,
    },
    "/api/status/:id" : { GET : verificationStatus, },
    "/health" : () => {
        return Response.json({ status: 'OK', timestamp: new Date().toISOString() });
    }

}
