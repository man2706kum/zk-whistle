import { connectDB } from "./config/database";
import { createVerification, verificationStatus } from "./controllers/verifyController";
import { Hono } from 'hono'
import { logger } from 'hono/logger'
import { cors } from 'hono/cors';
import { secureHeaders } from 'hono/secure-headers'

const PORT = Bun.env.PORT
const HOST = Bun.env.HOST

await connectDB();

const app = new Hono()
app.use(logger())
app.use(cors())
app.use(secureHeaders())

app.get('/health', (c) => c.json({ status: 'OK', timestamp: new Date().toISOString() }))

app.post('/api/verify', createVerification )
app.get('/api/status/:id', verificationStatus )

export default { 
  port: PORT, 
  fetch: app.fetch, 
} 