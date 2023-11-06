export interface MyInt {
    foo: string;
    bar: number | string;
    baar: "number" | "string";
    "foo bar": object;
    other: MyOtherType;
    oof: (number | string)[];
    rab: MyOtherType & string;
}

export type MyOtherType = {
    FOO: number[];
};

export function is_interface_MyInt(obj: unknown): obj is MyInt {
    return (
        typeof obj === "object" &&
        obj != null &&
        "foo" in obj &&
        typeof obj.foo === "string" &&
        "bar" in obj &&
        (typeof obj.bar === "number" || typeof obj.bar === "string") &&
        "foo bar" in obj &&
        typeof "foo bar" === "object" &&
        "other" in obj &&
        is_type_MyOtherType(obj.other)
    );
}

export function is_type_MyOtherType(obj: unknown): obj is MyOtherType {
    return (
        typeof obj === "object" &&
        obj != null &&
        "FOO" in obj &&
        Array.isArray(obj.FOO) &&
        arrayTypeCheck(obj.FOO, "number")
    );
}

export function arrayTypeCheckComplexTypes(arr: any[], typeChecker: (item: any) => boolean): boolean {
    for (let i = 0; i < arr.length; i++) {
        if (!typeChecker(arr[i])) {
            return false;
        }
    }
    return true;
}

export function arrayTypeCheck(
    arr: any[],
    type: "string" | "number" | "bigint" | "boolean" | "symbol" | "undefined" | "object" | "function"
): boolean {
    for (let i = 0; i < arr.length; i++) {
        if (typeof arr[i] !== type) {
            return false;
        }
    }
    return true;
}
