interface _0 {
    _1: "value";
    _2: (number | string)[];
    _3:
        | number
        | {
              _4: number;
              _5:
                  | string
                  | {
                        _6: { _7: "string" }[];
                    };
              _8: object;
              _9: "object";
          }
        | { _10: boolean };
    _11: _1;
    _12: Array<string | null> | object[];
    _13: Array<string>[];
}

interface _1 {
    _: (string[] | number[])[];
}
interface _2 {
    _: number | { _?: string };
}
interface _3 {
    _: Array<Array<number>>;
}
interface _4 {
    _: Array<Array<number>[] | string[][]>;
}
interface _5 {
    _: Array<number[] | (string[] & object)>;
}
interface _6 {
    _: { _?: "str" | 0 | 10_000 | number; __: undefined | null };
}
interface _7 {
    _: _6[];
}
interface _8 {
    _: _7[];
}
interface _9 {
    _: _8[];
}
interface _10 {
    _: _8[] | (_7[] & _4[]) | {};
}
interface _11 {
    _: [number, string, "dskfj", 1, _10];
}
interface _12 {
    _: _11 | _11[] | _11[][][][][][][];
}
interface _13 {
    _: _12 | _0;
}