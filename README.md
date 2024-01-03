# Typescript Runtime Typechecker

A trans-compiler that generates typechecker functions (`(o: unknown) => o is T`) and writes them at the given path as `.js` or `.ts` file based on interfaces in a given typescript file.

## Installation

Install the package globally with npm:

```bash
npm install --global ts-runtime-typechecker 
```

Or install with cargo:

```bash
cargo install ts-runtime-typechecker 
```

## Usage

```bash
ts-runtime-typechecker <READ-FILE-PATH> <WRITE-FILE-PATH>
```

## Features

### Supported

- Typescript common types:
  - `string` | `number` | `boolean` | `true` | `false`
  - `undefined` | `null` | `unknown` | `any`
  - `object` | `symbol` | `bigint`
- Javascript primitives:
  - string e.g. `"str"`
  - number e.g. `12_000`
- Arrays:
  - `T[]`
  - `Array<T>`
- Tuples:
  - `[T, U, P,...]`
- Operators:
  - `|`
  - `&`
- Generics or other types:
  - `Array<T>`
  - `Function`
- Interface declration merging

### Not Yet Supported

- Function types: e.g. `() => void`
- Indexed access types: e.g. `Foo["bar"]`
- Conditional types: e.g. `RegExp extends Foo ? number : string`
- Mapped types: e.g. `[key: string]: boolean;`
- Typescript utility types: e.g. `Required<T>`
- Some keywords: e.g. `keyof` | `typeof` | `extends` | `implements`

## Example

TypeScript interface `Foo` and the corresponding generated typechecker function `isFoo`.

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
        barbaz: [bigint, symbol];
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
