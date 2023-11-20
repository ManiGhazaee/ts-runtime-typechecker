export function isInter(o: unknown): o is Inter{return(o!=null&&typeof o==="object"&&Object.keys(o).length===2&&"_1"in o&&(typeof o["_1"]==="number"||(typeof o["_1"]==="string"&&typeof o["_1"]==="object"))&&"_2"in o&&typeof o["_2"]==="object"&&o["_2"]!=null&&Object.keys(o["_2"]).length===1&&"key_2"in o["_2"]&&(typeof o["_2"]["key_2"]==="string"||o["_2"]["key_2"]==="string"))}

export function isX(o: unknown): o is X{return(o!=null&&typeof o==="object"&&Object.keys(o).length===2&&"key"in o&&typeof o["key"]==="number"&&"key_2"in o&&(typeof o["key_2"]==="string"||o["key_2"]==="string"))}
