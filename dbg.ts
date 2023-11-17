interface MyInt {
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
    _11: AnotherType;
    _12: Array<string | null> | object[];
    _13: Array<string>[];
}

interface AnotherType {
    _1: number | string;
    _2: { _2_1: string; _2_2: number | string };
    _3: string;
    _4: { _4_1: { _4_1_1: string | (number & string) | number[]; _4_1_2: object[] }; _4_2: (string | number)[] };
}

interface X {
    string: "string";
    let: "stirng" | number;
    sdlkfj: {
        obj: { skfj: number[][] | string[][] };
        nu: "skfdj";
    };
    u: "skfj" | "array"[];
    k: {
        string: "string";
        let: "stirng" | number;
        sdlkfj: {
            obj: { skfj: number[][] | string[][] };
            nu: "skfdj";
        };
        u: "skfj" | "array"[];
    };
    x: {
        k: {
            string: "string";
            let: "stirng" | number;
            sdlkfj: {
                obj: { skfj: number[][] | string[][] };
                nu: "skfdj";
            };
            u: "skfj" | "array"[];
        };
        x: {
            string: "string";
            let: "stirng" | number;
            sdlkfj: {
                obj: { skfj: number[][] | string[][] };
                nu: "skfdj";
            };
            u: "skfj" | "array"[];
            k: {
                string: "string";
                let: "stirng" | number;
                sdlkfj: {
                    obj: { skfj: number[][] | string[][] };
                    nu: "skfdj";
                };
                u: "skfj" | "array"[];
            };
            x: {
                k: {
                    string: "string";
                    let: "stirng" | number;
                    sdlkfj: {
                        obj: { skfj: number[][] | string[][] };
                        nu: "skfdj";
                    };
                    u: "skfj" | "array"[];
                };
            };
        };
    };
    o: {
        string: "string";
        let: "stirng" | number;
        sdlkfj: {
            obj: { skfj: number[][] | string[][] };
            nu: "skfdj";
        };
        u: "skfj" | "array"[];
        k: {
            string: "string";
            let: "stirng" | number;
            sdlkfj: {
                obj: { skfj: number[][] | string[][] };
                nu: "skfdj";
            };
            u: "skfj" | "array"[];
        };
        x: {
            k: {
                string: "string";
                let: "stirng" | number;
                sdlkfj: {
                    obj: { skfj: number[][] | string[][] };
                    nu: "skfdj";
                };
                u: "skfj" | "array"[];
            };
        };
    };
}
