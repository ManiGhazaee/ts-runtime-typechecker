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

interface X {
    _1: string | number & object;
}
