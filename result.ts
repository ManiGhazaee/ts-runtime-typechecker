export function isInter(o: unknown): o is Inter{return(o!=null&&typeof o==="object"&&Object.keys(o).length===1&&"_1"in o&&(Array.isArray(o["_1"])&&(Array.isArray(o["_1"]["0"])&&(Array.isArray(o["_1"]["0"]["0"])&&o["_1"]["0"]["0"].length===1&&typeof o["_1"]["0"]["0"]["0"]==="object"&&o["_1"]["0"]["0"]["0"]!=null&&Object.keys(o["_1"]["0"]["0"]["0"]).length===1&&"_2"in o["_1"]["0"]["0"]["0"]&&(o["_1"]["0"]["0"]["0"]["_2"]===-20||typeof o["_1"]["0"]["0"]["0"]["_2"]==="function")))))}
