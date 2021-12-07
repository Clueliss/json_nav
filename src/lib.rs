use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum JsonNavError {
    #[error("could not navigate to {path}")]
    Navigation {
        path: &'static str
    },

    #[error("type mismatch, expected {expected}")]
    TypeMismatch {
        expected: &'static str,
    },
}

/// INTERNAL
/// The recursive implementation of the path walking and error message generation
#[doc(hidden)]
#[macro_export]
macro_rules! json_nav_internal {
    ($json:expr, $base_path:expr, $path:expr) => {
    	$json.and_then(|x| {
	    	x.get($path)
	    		.ok_or($crate::JsonNavError::Navigation { path: concat!($base_path, '.', $path) })
	    })
    };

    ($json:expr, $base_path:expr, $first_path:expr, $($path:expr),+) => {
        let _x = $crate::json_nav_internal!{ $json, $base_path, $first_path };
        $crate::json_nav_internal!{ _x, concat!($base_path, '.', $first_path), $($path),+ }
    };
}


#[doc = include_str!("../README.md")]
#[macro_export]
macro_rules! json_nav {
    ($json:expr => $($path:expr)=>+) => {
        {
    		$crate::json_nav_internal!{ Ok(&$json), stringify!($json), $($path),+ }
    	}
    };

    ($json:expr => $($path:expr)=>+; as object) => {
    	{
    		let _x = json_nav!{ $json => $($path)=>+ };
    		_x.and_then(|x| x.as_object().ok_or($crate::JsonNavError::TypeMismatch { expected: "object" }))
    	}
    };

    ($json:expr => $($path:expr)=>+; as array) => {
    	{
    		let _x = json_nav!{ $json => $($path)=>+ };
    		_x.and_then(|x| x.as_array().ok_or($crate::JsonNavError::TypeMismatch { expected: "array" }))
    	}
    };

    ($json:expr => $($path:expr)=>+; as str) => {
    	{
    		let _x = json_nav!{ $json => $($path)=>+ };
    		_x.and_then(|x| x.as_str().ok_or($crate::JsonNavError::TypeMismatch{ expected: "str" }))
    	}
    };

    ($json:expr => $($path:expr)=>+; as bool) => {
    	{
    		let _x = json_nav!{ $json => $($path)=>+ };
    		_x.and_then(|x| x.as_bool().ok_or($crate::JsonNavError::TypeMismatch{ expected: "bool" }))
    	}
    };

    ($json:expr => $($path:expr)=>+; as u64) => {
    	{
    		let _x = json_nav!{ $json => $($path)=>+ };
    		_x.and_then(|x| x.as_u64().ok_or($crate::JsonNavError::TypeMismatch{ expected: "u64" }))
    	}
    };

    ($json:expr => $($path:expr)=>+; as i64) => {
    	{
    		let _x = json_nav!{ $json => $($path)=>+ };
    		_x.and_then(|x| x.as_i64().ok_or($crate::JsonNavError::TypeMismatch{ expected: "i64" }))
    	}
    };

    ($json:expr => $($path:expr)=>+; as f64) => {
    	{
    		let _x = json_nav!{ $json => $($path)=>+ };
    		_x.and_then(|x| x.as_f64().ok_or($crate::JsonNavError::TypeMismatch{ expected: "f64" }))
    	}
    };
}
