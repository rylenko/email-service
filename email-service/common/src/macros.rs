#[macro_export]
macro_rules! const_assert {
	($($tt:tt)*) => {
		const _: () = assert!($($tt)*);
	};
}

/// [`println!`] your message if `common::consts::DEBUG` is `true`.
#[macro_export]
macro_rules! debug {
	($string:tt) => {
		if *common::consts::DEBUG {
			println!("[{}:{} at {}]: {}", line!(), column!(), file!(), $string);
		}
	};
	($string:tt, $($arg:tt)*) => {
		if *common::consts::DEBUG {
			let string = format!($string, $($arg)*);
			println!("[{}:{} at {}]: {}", line!(), column!(), file!(), string);
		}
	};
}

/// Works in the same way as [`vec!`], but is used to create a
/// [`std::collections::HashSet`].
#[macro_export]
macro_rules! set {
	($($x:expr),* $(,)?) => {
		{
			let mut set = std::collections::HashSet::new();
			$( set.insert($x); )*
			set
		}
	};
}

/// Creates a accessor function for struct field.
/// Args format: `operation field -> return_type`
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate common;
/// struct S<T> {
///     a: i32,
///     b: String,
///     c: Option<T>,
///     d: Option<String>,
/// }
/// impl<T> S<T> {
///     accessor!(copy a -> i32);
///     accessor!(& b -> &str);
///     accessor!(as_ref c -> Option<&T>);
///     accessor!(as_deref d -> Option<&str>);
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! accessor (
	(copy $field:ident -> $return_type:ty) => {
		#[inline]
		#[must_use]
		pub fn $field(&self) -> $return_type { self.$field }
	};
	(& $field:ident -> $return_type:ty) => {
		#[inline]
		#[must_use]
		pub fn $field(&self) -> $return_type { &self.$field }
	};
	(as_ref $field:ident -> $return_type:ty) => {
		#[inline]
		#[must_use]
		pub fn $field(&self) -> $return_type { self.$field.as_ref() }
	};
	(as_deref $field:ident -> $return_type:ty) => {
		#[inline]
		#[must_use]
		pub fn $field(&self) -> $return_type { self.$field.as_deref() }
	};
);

/// Just a shorthand for receiving packages (with debug), else doing
/// something.
///
/// # Example
///
/// ```no_run
/// # use std::net::SocketAddr;
/// # use common::{receive_package_or_else, set, package::Action};
/// # #[tokio::main]
/// # async fn main() -> Result<(), usize> {
/// #    let address = SocketAddr::from(([127, 0, 0, 1], 8888));
/// #    let mut stream
/// #        = common::connect_or_else!(address, None::<&str>, return Err(1));
/// let _package = receive_package_or_else!(
///     &mut stream,
///     address,
///     None,
///     Some(set![Action::SendEmail]),
///     return Err(2),
/// );
/// #    Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! receive_package_or_else {
	(
		$stream_mut_ref:expr,
		$address:expr,
		$password:expr,
		$accepted_actions:expr,
		$else:expr $(,)?
	) => {
		match common::package::Package::receive(
			$stream_mut_ref,
			$password,
			$accepted_actions,
		)
		.await
		{
			Ok(r) => {
				common::debug!("Received a valid package from {}.", $address);
				r
			}
			Err(e) => {
				common::debug!(
					"Failed to receive a package from {}: {}",
					$address,
					e
				);
				$else
			}
		}
	};
}

/// Just a shorthand for sending package, else execute some code.
#[macro_export]
macro_rules! send_package_or_else {
	($package:expr, $stream:expr, $address:expr, $else:expr $(,)?) => {
		match $package.send($stream).await {
			Ok(()) => {
				common::debug!(
					"The package was successfully sent to {}.",
					$address
				);
			}
			Err(e) => {
				common::debug!(
					"Failed to send the package to {}: {}",
					$address,
					e
				);
				$else
			}
		}
	};
}

/// Shortcut for creating connections.
///
/// # Example
///
/// ```no_run
/// # use std::net::SocketAddr;
/// # #[tokio::main]
/// # async fn main() -> Result<(), usize> {
/// #     let address = SocketAddr::from(([127, 0, 0, 1], 8888));
/// common::connect_or_else!(address, Some("127.0.0.1:7777"), return Err(1));
///     # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! connect_or_else {
	($address:expr, $proxy_option:expr, $else:expr $(,)?) => {
		match $proxy_option {
			Some(p) => {
				common::_connect_with_proxy_or_else!($address, p, $else)
			}
			None => common::_connect_without_proxy_or_else!($address, $else),
		}
	};
}

/// Just a shorthand for making connections without proxy, else returning
/// something.
#[macro_export]
macro_rules! _connect_without_proxy_or_else {
	($address:expr, $else:expr) => {{
		match tokio::net::TcpStream::connect($address).await {
			Ok(s) => s,
			Err(_) => common::_connect_failed!($address, $else),
		}
	}};
}

/// Same as `common::_connect_without_proxy` but with proxy support.
#[macro_export]
macro_rules! _connect_with_proxy_or_else {
	($address:expr, $proxy:expr, $else:expr) => {{
		let mut stream =
			common::_connect_without_proxy_or_else!($proxy, $else);
		if async_socks5::connect(&mut stream, $address, None).await.is_err() {
			common::_connect_failed!($address, $else);
		}
		stream
	}};
}

/// Used, for example, in `common::connect_or_else` if connection failed.
#[macro_export]
macro_rules! _connect_failed {
	($address:expr, $else:expr) => {{
		common::debug!("Failed to connect to {}.", $address);
		$else
	}};
}
