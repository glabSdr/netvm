use crate::buffer::{DATATYPE_CODE_BOOL, DATATYPE_CODE_BYTE, DATATYPE_CODE_CMD, DATATYPE_CODE_GRAPH, DATATYPE_CODE_LIST, DATATYPE_CODE_NUMBER, DATATYPE_CODE_POINTER, DATATYPE_CODE_STRING, DATATYPE_CODE_UINT, DATATYPE_CODE_UNDEFINED};
use crate::{Graph, List, Var};


impl Var {
    pub fn serialize(self) -> Vec<u8> {
        match self {
            Self::Undefined => {
                let mut buffer: Vec<u8> = Vec::new();
                buffer.push(DATATYPE_CODE_UNDEFINED);

                buffer
            }

            Var::Number(s) => {
                let mut buffer: Vec<u8> = Vec::new();
                buffer.push(DATATYPE_CODE_NUMBER);

                let bytes = s.to_be_bytes();
                buffer.append(&mut Vec::from(bytes));

                buffer
            }

            Var::Uint(s) => {
                let mut buffer: Vec<u8> = Vec::new();
                buffer.push(DATATYPE_CODE_UINT);

                let bytes = s.to_be_bytes();
                buffer.append(&mut Vec::from(bytes));

                buffer
            }

            Var::Pointer(s) => {
                let mut buffer: Vec<u8> = Vec::new();
                buffer.push(DATATYPE_CODE_POINTER);

                let bytes = s.to_be_bytes();
                buffer.append(&mut Vec::from(bytes));

                buffer
            }

            Var::String(s) => {
                let mut buffer: Vec<u8> = Vec::new();
                buffer.push(DATATYPE_CODE_STRING);
                buffer.append(&mut Vec::from(s.len().to_be_bytes()));
                buffer.append(&mut Vec::from(s));

                buffer
            }

            Var::Byte(s) => {
                let mut buffer: Vec<u8> = Vec::new();
                buffer.push(DATATYPE_CODE_BYTE);
                buffer.push(s);

                buffer
            }

            Var::Bool(s) => {
                let mut buffer: Vec<u8> = Vec::new();
                buffer.push(DATATYPE_CODE_BOOL);
                buffer.push(s as u8);

                buffer
            }


            Var::Cmd(s) => {
                let mut buffer: Vec<u8> = Vec::new();
                buffer.push(DATATYPE_CODE_CMD);
                buffer.push(s);

                buffer
            }
        }
    }
}

impl List {
    pub fn serialize(self) -> Vec<u8> {
        let keys = self.clone().get_keys();
        let mut buffer: Vec<u8> = Vec::new();
        buffer.push(DATATYPE_CODE_LIST);
        buffer.append(&mut Vec::from(keys.len().to_be_bytes()));

        for key in keys {
            let value = self.0.get(&key).unwrap();
            buffer.append(&mut Vec::from(key));
            buffer.append(&mut value.clone().serialize());
        }
        buffer
    }
}


impl Graph {
    pub fn serialize(self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.push(DATATYPE_CODE_GRAPH);
        buffer.append(&mut Vec::from(self.0.len().to_be_bytes()));

        for list in self.0 {
            let list_buffer = list.clone().serialize();
            buffer.append(&mut Vec::from(list_buffer.len().to_be_bytes()));
            buffer.append(&mut list.serialize());
        }

        buffer
    }

}
