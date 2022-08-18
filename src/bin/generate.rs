use std::io::Write;

fn generate() -> String {
    let mut buffer = Vec::new();
    for i in 0..100 {
        let i = i + 1;
        write!(&mut buffer, "impl<").unwrap();
        for j in 0..i {
            write!(&mut buffer, "T{}: 'static,", j).unwrap();
        }
        write!(&mut buffer, "> TupleUnpack for (").unwrap();
        for j in 0..i {
            write!(&mut buffer, "T{},", j).unwrap();
        }
        writeln!(&mut buffer, ") {{").unwrap();
        writeln!(&mut buffer, "    fn unpack_types(&self) -> Vec<TypeId> {{",).unwrap();
        write!(&mut buffer, "        vec![").unwrap();
        for j in 0..i {
            write!(&mut buffer, "TypeId::of::<T{}>(),", j).unwrap();
        }
        writeln!(&mut buffer, "]").unwrap();
        writeln!(&mut buffer, "    }}").unwrap();
        writeln!(&mut buffer, "    fn unpack_data(&self) -> Vec<&dyn Any> {{",).unwrap();
        write!(&mut buffer, "        vec![").unwrap();
        for j in 0..i {
            write!(&mut buffer, "&self.{},", j).unwrap();
        }
        writeln!(&mut buffer, "]").unwrap();
        writeln!(&mut buffer, "    }}").unwrap();
        writeln!(&mut buffer, "}}").unwrap();
    }
    unsafe { String::from_utf8_unchecked(buffer) }
}

pub fn main() {
    let code = generate();
    println!("{}", code);
}
