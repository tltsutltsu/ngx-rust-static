/// Utility function to provide typed checking of the mask's field state.
#[inline(always)]
pub fn check_mask(mask: DebugMask, log_level: usize) -> bool {
    let mask_bits: u32 = mask.into();
    if log_level & mask_bits as usize == 0 {
        return false;
    }
    true
}

/// Write to logger at a specified level.
///
/// See [Logging](https://nginx.org/en/docs/dev/development_guide.html#logging)
/// for available log levels.
#[macro_export]
macro_rules! ngx_log_error {
    ( $level:expr, $log:expr, $($arg:tt)+ ) => {
        let log = $log;
        let level = $level as $crate::ffi::ngx_uint_t;
        if level < unsafe { (*log).log_level } {
            let message = ::std::format!($($arg)+);
            let message = message.as_bytes();
            unsafe {
                $crate::ffi::ngx_log_error_core(level, log, 0, c"%*s".as_ptr(), message.len(), message.as_ptr());
            }
        }
    }
}

/// Write to logger with the context of currently processed configuration file.
#[macro_export]
macro_rules! ngx_conf_log_error {
    ( $level:expr, $cf:expr, $($arg:tt)+ ) => {
        let cf: *mut $crate::ffi::ngx_conf_t = $cf;
        let level = $level as $crate::ffi::ngx_uint_t;
        if level < unsafe { (*(*cf).log).log_level } {
            let message = ::std::format!($($arg)+);
            let message = message.as_bytes();
            unsafe {
                $crate::ffi::ngx_conf_log_error(level, cf, 0, c"%*s".as_ptr(), message.len(), message.as_ptr());
            }
        }
    }
}

/// Write to logger at debug level.
#[macro_export]
macro_rules! ngx_log_debug {
    ( mask: $mask:expr, $log:expr, $($arg:tt)+ ) => {
        let log = $log;
        if $crate::log::check_mask($mask, unsafe { (*log).log_level }) {
            let level = $crate::ffi::NGX_LOG_DEBUG as $crate::ffi::ngx_uint_t;
            let message = format!($($arg)+);
            let message = message.as_bytes();
            unsafe {
                $crate::ffi::ngx_log_error_core(level, log, 0, c"%*s".as_ptr(), message.len(), message.as_ptr());
            }
        }
    };
    ( $log:expr, $($arg:tt)+ ) => {
        $crate::ngx_log_debug!(mask: $crate::log::DebugMask::All, $log, $($arg)+);
    }
}

/// Log to request connection log at level [`NGX_LOG_DEBUG_HTTP`].
///
/// [`NGX_LOG_DEBUG_HTTP`]: https://nginx.org/en/docs/dev/development_guide.html#logging
#[macro_export]
macro_rules! ngx_log_debug_http {
    ( $request:expr, $($arg:tt)+ ) => {
        let log = unsafe { (*$request.connection()).log };
        $crate::ngx_log_debug!(mask: $crate::log::DebugMask::Http, log, $($arg)+);
    }
}

/// Log with requested debug mask.
///
/// **NOTE:** This macro supports [`DebugMask::Http`] (`NGX_LOG_DEBUG_HTTP`), however, if you have
/// access to a Request object via an http handler it can be more convenient and readable to use
/// the [`ngx_log_debug_http`] macro instead.
///
/// See <https://nginx.org/en/docs/dev/development_guide.html#logging> for details and available
/// masks.
#[macro_export]
macro_rules! ngx_log_debug_mask {
    ( DebugMask::Core, $log:expr, $($arg:tt)+ ) => {
        $crate::ngx_log_debug!(mask: $crate::log::DebugMask::Core, $log, $($arg)+);
    };
    ( DebugMask::Alloc, $log:expr, $($arg:tt)+ ) => {
        $crate::ngx_log_debug!(mask: $crate::log::DebugMask::Alloc, $log, $($arg)+);
    };
    ( DebugMask::Mutex, $log:expr, $($arg:tt)+ ) => {
        $crate::ngx_log_debug!(mask: $crate::log::DebugMask::Mutex, $log, $($arg)+);
    };
    ( DebugMask::Event, $log:expr, $($arg:tt)+ ) => {
        $crate::ngx_log_debug!(mask: $crate::log::DebugMask::Event, $log, $($arg)+);
    };
    ( DebugMask::Http, $log:expr, $($arg:tt)+ ) => {
        $crate::ngx_log_debug!(mask: $crate::log::DebugMask::Http, $log, $($arg)+);
    };
    ( DebugMask::Mail, $log:expr, $($arg:tt)+ ) => {
        $crate::ngx_log_debug!(mask: $crate::log::DebugMask::Mail, $log, $($arg)+);
    };
    ( DebugMask::Stream, $log:expr, $($arg:tt)+ ) => {
        $crate::ngx_log_debug!(mask: $crate::log::DebugMask::Stream, $log, $($arg)+);
    };
}

/// Debug masks for use with [`ngx_log_debug_mask`], these represent the only accepted values for
/// the mask.
#[derive(Debug)]
pub enum DebugMask {
    /// Aligns to the NGX_LOG_DEBUG_CORE mask.
    Core,
    /// Aligns to the NGX_LOG_DEBUG_ALLOC mask.
    Alloc,
    /// Aligns to the NGX_LOG_DEBUG_MUTEX mask.
    Mutex,
    /// Aligns to the NGX_LOG_DEBUG_EVENT mask.
    Event,
    /// Aligns to the NGX_LOG_DEBUG_HTTP mask.
    Http,
    /// Aligns to the NGX_LOG_DEBUG_MAIL mask.
    Mail,
    /// Aligns to the NGX_LOG_DEBUG_STREAM mask.
    Stream,
    /// Aligns to the NGX_LOG_DEBUG_ALL mask.
    All,
}

impl TryFrom<u32> for DebugMask {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            crate::ffi::NGX_LOG_DEBUG_CORE => Ok(DebugMask::Core),
            crate::ffi::NGX_LOG_DEBUG_ALLOC => Ok(DebugMask::Alloc),
            crate::ffi::NGX_LOG_DEBUG_MUTEX => Ok(DebugMask::Mutex),
            crate::ffi::NGX_LOG_DEBUG_EVENT => Ok(DebugMask::Event),
            crate::ffi::NGX_LOG_DEBUG_HTTP => Ok(DebugMask::Http),
            crate::ffi::NGX_LOG_DEBUG_MAIL => Ok(DebugMask::Mail),
            crate::ffi::NGX_LOG_DEBUG_STREAM => Ok(DebugMask::Stream),
            crate::ffi::NGX_LOG_DEBUG_ALL => Ok(DebugMask::All),
            _ => Err(0),
        }
    }
}

impl From<DebugMask> for u32 {
    fn from(value: DebugMask) -> Self {
        match value {
            DebugMask::Core => crate::ffi::NGX_LOG_DEBUG_CORE,
            DebugMask::Alloc => crate::ffi::NGX_LOG_DEBUG_ALLOC,
            DebugMask::Mutex => crate::ffi::NGX_LOG_DEBUG_MUTEX,
            DebugMask::Event => crate::ffi::NGX_LOG_DEBUG_EVENT,
            DebugMask::Http => crate::ffi::NGX_LOG_DEBUG_HTTP,
            DebugMask::Mail => crate::ffi::NGX_LOG_DEBUG_MAIL,
            DebugMask::Stream => crate::ffi::NGX_LOG_DEBUG_STREAM,
            DebugMask::All => crate::ffi::NGX_LOG_DEBUG_ALL,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_mask_lower_bound() {
        assert!(<DebugMask as Into<u32>>::into(DebugMask::Core) == crate::ffi::NGX_LOG_DEBUG_FIRST);
    }
    #[test]
    fn test_mask_upper_bound() {
        assert!(<DebugMask as Into<u32>>::into(DebugMask::Stream) == crate::ffi::NGX_LOG_DEBUG_LAST);
    }
    #[test]
    fn test_check_mask() {
        struct MockLog {
            log_level: usize,
        }
        let mock = MockLog { log_level: 16 };

        let mut r = check_mask(DebugMask::Core, mock.log_level);
        assert!(r);

        r = check_mask(DebugMask::Alloc, mock.log_level);
        assert!(!r);
    }
}
