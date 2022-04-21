use serde::{Deserialize, Serialize};

pub enum Exception {
  BadRequest,
  Unauthorized,
  NotFound,
  Forbidden,
  NotAcceptable,
  RequestTimeout,
  Conflict,
  Gone,
  HttpVersionNotSupported,
  PayloadTooLarge,
  UnsupportedMediaType,
  UnprocessableEntity,
  InternalServerError,
  NotImplemented,
  ImATeapot,
  MethodNotAllowed,
  BadGateway,
  ServiceUnavailable,
  GatewayTimeout,
  PreconditionFailed,
}
