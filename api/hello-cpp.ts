import type { VercelRequest, VercelResponse } from '@vercel/node'
import { sayHello } from 'hello';

export default function handler(req: VercelRequest, res: VercelResponse) {
  const { name = 'World' } = req.query
  return res.json({
    message: sayHello(`Hello ${name}!`),
  })
}