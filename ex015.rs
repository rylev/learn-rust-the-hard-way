use std::io::{File,FileAccess,IoResult,SeekStyle};
use std::io::FileMode::Open;
use std::io::FileAccess::{Write,ReadWrite};
use std::fmt::{Show,Formatter,Error};

// Let's build a database

// Set some constants. Note that the data type must be explicitly given
const MAX_DATA: u16 = 512;
const MAX_ROWS: u8 = 100;

struct Field {
    field: [u8, ..MAX_DATA as uint]
}

struct Row {
    id: u8,
    set: u8,
    name: Field,
    email: Field
}

struct Database {
    rows: [Row, ..MAX_ROWS as uint]
}

struct Connection {
    file: File,
    database: Database
}

fn main() {
    // Get the command line arguments and the length of those arguments
    let argv = std::os::args();
    let argc = argv.len();

    // We do a simple check here that the right number of args have been given
    if argc < 3 {
        panic!("USAGE: ex15 <dbfile> <action> [action params]");
    }

    let filename = argv[1].as_slice();
    let action = argv[2].as_slice();
    // As you will see we will be mutating part of our connection so we need to mark
    // it as mutable
    let mut conn = Connection::new(filename, action);

    // we must provide the type of id so that the from_str() function can cast to
    // the right type
    let id: u8 = if argc > 3 {
        let arg = argv[3].as_slice();
        // patterning matching against the option type
        // Option can either be Some or None. Many languages use nil or null to represent
        // the idea of nothingness. Rust does not support a null pointer, but rather
        // the idea of an optional value.
        match from_str(arg) {
            Some(i) => i,
            None => panic!("{} is not a number!", arg)
        }
    } else {
        0
    };

    if id >= (MAX_ROWS) {
        panic!("There's not that many records.");
    }

    match action.as_slice() {
        "c" => {
            conn.database.create();
            conn.write();
        },
        "g" => {
            if argc != 4 { panic!("Need an id to get"); }
            conn.database.get(id);
        },
        "s" => {
            if argc != 6 { panic!("Need id, name, email to set"); }

            conn.database.set(id, argv[4].as_slice(), argv[5].as_slice());
            conn.write();
        },
        "d" => {
            if argc != 4 { panic!("Need id to delete"); }

            conn.database.delete(id);
            conn.write();
        },
        "l" => {
            conn.database.list();
        },
        _ => panic!("Invalid action, only: c=create, g=get, s=set, d=del, l=list")
    }
}

impl Field {
    fn empty() -> Field {
        // the unsafe keyword allows us to do things that are not memory safe. By default
        // rust is a completely memory safe language. Unlike C or C++ Rust does not
        // allow you to arbitrarily manipulate memory. However, if you need to do so,
        // you can use the unsafe keyword. Here we zero out the memory of the field
        unsafe {
            Field { field: std::mem::zeroed() }
        }
    }
}

impl Show for Field {
    // to implement the show trait for an struct you must implement the fmt function
    // fmt requires you to write some Formatter.
    fn fmt(&self, formater: &mut Formatter) -> Result<(), Error> {
        write!(formater, "{}", self.field.to_ascii().as_str_ascii())
    }
}

impl Row {
    fn empty() -> Row {
        Row { id: 0, set: 0, name: Field::empty(), email: Field::empty() }
    }

    fn from_reader(&mut self, file: &mut File) -> IoResult<()> {
        // the try! macro performs the computation passed to it and returns a
        // failure from the method if the computation fails
        // this means that if the call to file.read_u8() fails, the entire function
        // will fail too and none of the following lines will be executed
        self.id = try!(file.read_u8());
        self.set = try!(file.read_u8());
        try!(file.read(self.name.field.as_mut_slice()));
        try!(file.read(self.email.field.as_mut_slice()));
        Ok(())
    }

    fn to_writer(&mut self, file: &mut File) -> IoResult<()> {
        try!(file.write_u8(self.id));
        try!(file.write_u8(self.set));
        try!(file.write(self.name.field.as_slice()));
        try!(file.write(self.email.field.as_slice()));
        Ok(())
    }

    fn set(&mut self, name: &str, email: &str) {
        if self.set == 1  {
            panic!("already set, must first delete");
        }
        self.set = 1;

        let mut name_copy = [0,..512];
        std::slice::bytes::copy_memory(&mut name_copy, name.as_bytes());
        self.name = Field { field: name_copy };

        let mut email_copy = [0,..512];
        std::slice::bytes::copy_memory(&mut email_copy, email.as_bytes());
        self.email = Field { field: email_copy };
    }

    fn print(&self) {
        println!("<{}> {} {}", self.id, self.name, self.email);
    }
}

impl Show for Row {
    fn fmt(&self, formater: &mut Formatter) -> Result<(), Error> {
        write!(formater, "<{}> {} {}", self.id, self.name, self.email)
    }
}

impl Database {
    fn empty() -> Database {
        Database {
            rows: [Row::empty(), ..MAX_ROWS as uint]
        }
    }

    fn create(&mut self) {
        for i in range(0, MAX_ROWS) {
            self.rows[i as uint].id = i as u8;
        }
    }

    fn set(&mut self, id: u8, name: &str, email: &str) {
        self.rows[id as uint].set(name, email);
    }

    fn list(&self) {
        for row in self.rows.iter() {
            if row.set == 1 {
                row.print();
            }
        }

    }

    fn get(&self, id: u8) {
        let row = &self.rows[id as uint];
        if row.set == 1 { row.print(); } else { panic!("ID is not set"); }
    }

    fn delete(&mut self, id: u8) {
        self.rows[id as uint] = Row::empty();
    }
}

impl Connection {
    fn new(filename: &str, mode: &str) -> Connection {
        let path = &Path::new(filename);
        if mode == "c" {
            let file = Connection::new_file(path, Write);
            Connection { file: file, database: Database::empty() }
        } else {
            let file = Connection::new_file(path, ReadWrite);
            let mut conn = Connection {file: file, database: Database::empty()};
            conn.load();
            conn
        }
    }

    fn new_file(path: &Path, access: FileAccess) -> File {
        match File::open_mode(path, Open, access) {
            Ok(file) => file,
            Err(e) => panic!("file error: {}", e)

        }
    }

    fn load(&mut self) {
        for row in self.database.rows.iter_mut() {
            // use return value to know when to stop
            match row.from_reader(&mut self.file) {
                Ok(_) => (),
                _ => return ()

            };
        }
    }

    fn write(&mut self) {
        match self.file.seek(0, SeekStyle::SeekSet) {
            Ok(_) => (),
            _ => return ()

        };
        for row in self.database.rows.iter_mut() {
            match row.to_writer(&mut self.file) {
                Ok(_) => (),
                _ => return ()

            };
        }
    }
}
