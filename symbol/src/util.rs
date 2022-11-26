use symbolic_common::Name;
use symbolic_demangle::{Demangle,DemangleOptions};

pub fn demangle_name(origin_name:&str) ->String{
    let name = Name::from(origin_name);
    let result = name.try_demangle(DemangleOptions::return_type(DemangleOptions::complete(), false));
    result.to_string()
}