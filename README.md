# Typescript Runtime Typechecker

## Features

- Typescript common types: 
  - [x] `string` | `number` | `boolean` | `true` | `false`
  - [x] `undefined` | `null` | `unknown` | `any`
  - [x] `object` | `symbol` | `bigint` 
- Javascript primitives:
  - [x] string e.g. `"str"`
  - [x] number e.g. `12_000`
  - [x] `null`
  - [ ] `symbol("foo")`
- Arrays:
  - [x] `T[]`
  - [x] `Array<T>`
- Tuples:
  - [x] `[T, U, P,...]`
- Operators:
  - [x] `|`
  - [x] `&`
- Generics or other types: 
  - [x] `Array<T>`
  - [x] `Function`
  - [ ] anything else
- Keywords:
  - [x] `interface`
  - [ ] `type`
  - [ ] `keyof`
  - [ ] `typeof`
  - [ ] `extends`
  - [ ] `implements`
- [x] Interface declration merging 
- [ ] Function types: e.g. `() => void` | `(): void`
- [ ] Indexed access types: e.g. `Foo["bar"]` 
- [ ] Conditional types: e.g. `RegExp extends Foo ? number : string` 
- [ ] Mapped types: e.g. `[key: string]: boolean;` 
- [ ] Typescript utility types: e.g. `Required<T>` 

## Examples

```typescript
interface Foo {
    foo: string | number;
    bar: "str" | "";
    foobar: 0 | 100_000;
    baz: number[] | null | undefined;
    qux: {
        faz: Array<number>;
        boo: (number | "str")[][];
        foobaz: object;
        barbaz: [bigint, unknown, symbol];
    };
}
```

```typescript
export function isFoo(o: unknown): o is Foo {
    return (
        o != null &&
        typeof o === "object" &&
        Object.keys(o).length === 5 &&
        "foo" in o &&
        (typeof o["foo"] === "string" || typeof o["foo"] === "number") &&
        "bar" in o &&
        (o["bar"] === "str" || o["bar"] === "") &&
        "foobar" in o &&
        (o["foobar"] === 0 || o["foobar"] === 100000) &&
        "baz" in o &&
        ((Array.isArray(o["baz"]) && typeof o["baz"]["0"] === "number") ||
            o["baz"] === null ||
            typeof o["baz"] === "undefined") &&
        "qux" in o &&
        typeof o["qux"] === "object" &&
        o["qux"] != null &&
        Object.keys(o["qux"]).length === 4 &&
        "faz" in o["qux"] &&
        Array.isArray(o["qux"]["faz"]) &&
        typeof o["qux"]["faz"]["0"] === "number" &&
        "boo" in o["qux"] &&
        Array.isArray(o["qux"]["boo"]) &&
        (typeof o["qux"]["boo"]["0"] === "number" ||
            (Array.isArray(o["qux"]["boo"]["0"]) && o["qux"]["boo"]["0"]["0"] === "str")) &&
        "foobaz" in o["qux"] &&
        typeof o["qux"]["foobaz"] === "object" &&
        "barbaz" in o["qux"] &&
        Array.isArray(o["qux"]["barbaz"]) &&
        o["qux"]["barbaz"].length === 2 &&
        typeof o["qux"]["barbaz"]["0"] === "bigint" &&
        typeof o["qux"]["barbaz"]["1"] === "symbol"
    );
}
```
