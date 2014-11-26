use std::io::File;
use std::io::FileMode::Open;
use std::io::FileAccess::{Write,ReadWrite};

const MAX_DATA: uint = 512;
const MAX_ROWS: uint = 100;


type Field = [u8, ..MAX_DATA];

struct Row {
    id: i64,
    set: i64,
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

const EMPTY_FIELD: Field = [0, ..MAX_DATA];
const EMPTY_ROW: Row = Row {id: 0, set: 0, name: EMPTY_FIELD, email: EMPTY_FIELD};
const EMPTY_DATABASE: Database = Database {rows: [EMPTY_ROW, ..MAX_ROWS]};

//impl Address {
//    fn print(&self) {
//        print!("{} {} {}", self.id, self.name, self.email);
//    }
//}

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

            let conn = Connection {file: file, database: EMPTY_DATABASE};
            conn.load_database();
            conn
        }
    }

    fn load_database(&self) {
        //int rc = fread(conn->db, sizeof(struct Database), 1, conn->file);
        //if(rc != 1) panic!("Failed to load database.");
    }

    fn create_database(&self) {
    }
}


//impl Database {
//    fn get(&self, id: u8) {
//        let addr = &db.rows[id];
//
//        if addr.set  {
//            addr.print();
//        } else {
//            panic!("ID is not set");
//        }
//    }
//}


//void Database_write(struct Connection *conn)
//{
//    rewind(conn->file);
//
//    int rc = fwrite(conn->db, sizeof(struct Database), 1, conn->file);
//    if(rc != 1) die("Failed to write database.");
//
//    rc = fflush(conn->file);
//    if(rc == -1) die("Cannot flush database.");
//}
//
//void Database_create(struct Connection *conn)
//{
//    int i = 0;
//
//    for(i = 0; i < MAX_ROWS; i++) {
//            // make a prototype to initialize it
//            struct Address addr = {.id = i, .set = 0};
//            // then just assign it
//            conn->db->rows[i] = addr;
//        }
//}
//
//void Database_set(struct Connection *conn, int id, const char *name, const char *email)
//{
//    struct Address *addr = &conn->db->rows[id];
//    if(addr->set) die("Already set, delete it first");
//
//    addr->set = 1;
//    // WARNING: bug, read the "How To Break It" and fix this
//    char *res = strncpy(addr->name, name, MAX_DATA);
//    // demonstrate the strncpy bug
//    if(!res) die("Name copy failed");
//
//    res = strncpy(addr->email, email, MAX_DATA);
//    if(!res) die("Email copy failed");
//}
//
//
//void Database_delete(struct Connection *conn, int id)
//{
//    struct Address addr = {.id = id, .set = 0};
//    conn->db->rows[id] = addr;
//}
//
//void Database_list(struct Connection *conn)
//{
//    int i = 0;
//    struct Database *db = conn->db;
//
//    for(i = 0; i < MAX_ROWS; i++) {
//            struct Address *cur = &db->rows[i];
//
//            if(cur->set) {
//                        Address_print(cur);
//                    }
//        }
//}

fn main() {
    let argv = std::os::args();
    let argc = argv.len();

    if argc < 3 {
        panic!("USAGE: ex15 <dbfile> <action> [action params]");
    }

    let filename = &argv[1];
    let action = &argv[2];
    let conn = Connection::new(filename.as_slice(), action.as_slice());

    let id: uint = match from_str(argv[3].as_slice()) {
        Some(i) => i,
        None => panic!("{} is not a number!")
    };

    if id >= MAX_ROWS {
        panic!("There's not that many records.");
    }

    // Remove me!
    conn.create_database();
    //match action {
    //    "c" => {
    //        conn.create_database();
    //        conn.write_database();
    //    },
    //    "g" => {
    //        if argc != 4 { panic!("Need an id to get"); }
    //        let database = &conn.database;
    //        database.get(id);
    //    },
    //    "s" => {
    //        if argc != 6 { panic!("Need id, name, email to set"); }

    //        let database = &conn.database;

    //        database.set(id, argv[4], argv[5]);
    //        conn.write_database();
    //    },

    //    "d" => {
    //        if argc != 4 { panic!("Need id to delete"); }

    //        let database = &conn.database;

    //        database.delete(id);
    //        conn.write_database();
    //    },
    //    "l" => {
    //        conn.list_database();
    //    },
    //    _ => ("Invalid action, only: c=create, g=get, s=set, d=del, l=list")
    //}

    //conn.close_database();
}