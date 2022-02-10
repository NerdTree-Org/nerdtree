export type StatusPayload<T = {}> = {
  success: boolean
  message?: string
  value?: T
}
