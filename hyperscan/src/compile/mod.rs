mod error;
#[macro_use]
mod pattern;
mod builder;
mod expr;
#[macro_use]
mod literal;
mod platform;

pub use self::builder::Builder;
pub use self::error::{AsCompileResult, Error};
#[doc(hidden)]
#[deprecated = "use `ExprInfo` instead"]
pub use self::expr::ExprInfo as ExpressionInfo;
pub use self::expr::ExprInfo;
pub use self::literal::{Flags as LiteralFlags, Literal, Literals};
pub use self::pattern::{Ext as ExpressionExt, Flags, Pattern, Patterns, SomHorizon};
pub use self::platform::{CpuFeatures, Platform, PlatformRef, Tune};
