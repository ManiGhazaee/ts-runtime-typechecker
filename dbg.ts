// interface MyInt {
//     _1: "value";
//     _2: (number | string)[];
//     _3:
//         | number
//         | {
//               _4: number;
//               _5:
//                   | string
//                   | {
//                         _6: { _7: "string" }[];
//                     };
//               _8: object;
//               _9: "object";
//           }
//         | { _10: boolean };
//     _11: AnotherType;
//     _12: Array<string | null> | object[];
//     _13: Array<string>[];
// }

interface AnotherType {
    key_2: Array<number[] | string> | { key_3: (Array<number[]> | number)[] };
}

// interface X {
//     _1: string[] | number & object;
// }

// function isAnotherType(obj: unknown): obj is AnotherType {
//     return (
//         obj != null &&
//         typeof obj === "object" &&
//         "key_2" in obj &&
//         ((Array.isArray(obj["key_2"]) &&
//             ((Array.isArray(obj["key_2"]["0"]) && typeof obj["key_2"]["0"]["0"] === "number") ||
//                 typeof obj["key_2"]["0"] === "string")) ||
//             (typeof obj["key_2"] === "object" &&
//                 obj["key_2"] != null &&
//                 "key_3" in obj["key_2"] &&
//                 Array.isArray(obj["key_2"]["key_3"]) &&
//                 ((Array.isArray(obj["key_2"]["key_3"]["0"]) &&
//                     Array.isArray(obj["key_2"]["key_3"]["0"]["0"]) &&
//                     typeof obj["key_2"]["key_3"]["0"]["0"]["0"] === "number") ||
//                     typeof obj["key_2"]["key_3"]["0"] === "number")))
//     );
// }
