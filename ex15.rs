use std::io::File; use std::io::FileMode::Open;
use std::io::FileAccess::{Write,ReadWrite};
use std::io::IoResult;
use std::fmt::{Show,Formatter,Error};
use std::io::SeekStyle;

const MAX_DATA: uint = 512;
const MAX_ROWS: uint = 100;


struct Field {
    field: [u8, ..MAX_DATA]
}

struct Row {
    id: u64,
    set: u64,
    name: Field,
    email: Field
}

struct Database {
    rows: [Row, ..MAX_ROWS]
}

struct Connection {
    file: File,
    database: Database
}

const EMPTY_FIELD: Field = Field { field: [0, ..MAX_DATA] };
const EMPTY_ROW: Row = Row {id: 0, set: 0, name: EMPTY_FIELD, email: EMPTY_FIELD};
const EMPTY_DATABASE: Database = Database {rows: [EMPTY_ROW, ..MAX_ROWS]};

impl Show for Field {
    fn fmt(&self, formater: &mut Formatter) -> Result<(), Error> {
        // TODO: encode the field field of field
        // http://doc.rust-lang.org/std/ascii/trait.AsciiCast.html
        write!(formater, "FIXME!")
    }
}

impl Row {
    fn from_reader(&mut self, file: &mut File) -> IoResult<()> {
        self.id = try!(file.read_le_u64());
        self.set = try!(file.read_le_u64());
        try!(file.read(self.name.field.as_mut_slice()));
        try!(file.read(self.email.field.as_mut_slice()));
        Ok(())
    }

    fn to_writer(&mut self, file: &mut File) -> IoResult<()> {
        try!(file.write_le_u64(self.id));
        try!(file.write_le_u64(self.set));
        try!(file.write(self.name.field.as_slice()));
        try!(file.write(self.email.field.as_slice()));
        Ok(())
    }

    fn set(&mut self, name: &str, email: &str) {
        if (self.set == 1) {
            panic!("already set, must first delete");
        }
        self.set = 1;
        // need to check the length of the string
        let mut name_copy = [0,..512];
        std::slice::bytes::copy_memory(&mut name_copy, name.as_bytes());
        self.name = Field { field: name_copy };

        let mut email_copy = [0,..512];
        std::slice::bytes::copy_memory(&mut email_copy, email.as_bytes());
        self.email = Field { field: email_copy };
    }
}

impl Show for Row {
    fn fmt(&self, formater: &mut Formatter) -> Result<(), Error> {
        write!(formater, "<{}> {} {}", self.id, self.name, self.email)
    }
}

impl Database {
    fn set(&mut self, id: u64, name: &str, email: &str) {
        self.rows[id as uint].set(email, email);
    }
}

impl Connection {
    fn new(filename: &str, mode: &str) -> Connection {
        let path = Path::new(filename);
        if mode == "c" {
            let file = match File::open_mode(&path, Open, Write){
                Ok(file) => file,
                Err(e) => panic!("file error: {}", e)
            };
            Connection { file: file, database: EMPTY_DATABASE }
        } else {
            let file = match File::open_mode(&path, Open, ReadWrite){
                Ok(file) => file,
                Err(e) => panic!("file error: {}", e)
            };

            let mut conn = Connection {file: file, database: EMPTY_DATABASE};
            conn.load_database();
            conn
        }
    }

    fn load_database(&mut self) {
        for row in self.database.rows.iter_mut() {
            // use return value to know when to stop
            row.from_reader(&mut self.file);
        }
    }

    fn create_database(&mut self) {
        for i in range(0, MAX_ROWS) {
            for row in self.database.rows.iter_mut() {
                row.id = i as u64;
            }
        }
    }

    fn write_database(&mut self) {
        self.file.seek(0, SeekStyle::SeekSet);
        for row in self.database.rows.iter_mut() {
            row.to_writer(&mut self.file);
        }
    }

}

fn main() {
    let argv = std::os::args();
    let argc = argv.len();

    if argc < 3 {
        panic!("USAGE: ex15 <dbfile> <action> [action params]");
    }

    let filename = argv[1].as_slice();
    let action = argv[2].as_slice();
    let mut conn = Connection::new(filename, action);

    let id: u64 = match from_str(argv[3].as_slice()) {
        Some(i) => i,
        None => panic!("{} is not a number!")
    };

    if id >= (MAX_ROWS as u64) {
        panic!("There's not that many records.");
    }

    match action.as_slice() {
        "c" => {
            conn.create_database();
            conn.write_database();
        },
        //"g" => {
        //    if argc != 4 { panic!("Need an id to get"); }
        //    let database = &conn.database;
        //    database.get(id);
        //},
        "s" => {
            if argc != 6 { panic!("Need id, name, email to set"); }

            conn.database.set(id, argv[4].as_slice(), argv[5].as_slice());
            conn.write_database();
        },

        //"d" => {
        //    if argc != 4 { panic!("Need id to delete"); }

        //    let database = &conn.database;

        //    database.delete(id);
        //    conn.write_database();
        //},
        //"l" => {
        //    conn.list_database();
        //},
        _ => panic!("Invalid action, only: c=create, g=get, s=set, d=del, l=list")
    }

}