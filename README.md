# Typescript Runtime Typechecker

A command line tool that generates TypeScript runtime typechecker functions based on interfaces in a given TypeScript file. These typechecker functions can be utilized to validate the structure and types of objects at runtime, enhancing type safety in your TypeScript projects.

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

Run the tool by providing the path to the input TypeScript file and the desired output file path:

```bash
ts-runtime-typechecker <INPUT-FILE-PATH> <OUTPUT-FILE-PATH>
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
// input file:
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
// output file:
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

___

TypeScript interfaces `Bar`, `Baz` and the corresponding generated typechecker functions `isBar`, `isBaz`.

```typescript
// input file:
interface Bar {
    foo: number | Baz;
}

interface Baz {
    foo: string;
}

interface Bar {
    bar: [Baz]
}
```

```typescript
// output file:
export function isBar(o: unknown): o is Bar {
    return (
        o != null &&
        typeof o === "object" &&
        Object.keys(o).length === 2 &&
        "foo" in o &&
        (typeof o["foo"] === "number" ||
            (typeof o["foo"] === "object" &&
                o["foo"] != null &&
                Object.keys(o["foo"]).length === 1 &&
                "foo" in o["foo"] &&
                typeof o["foo"]["foo"] === "string")) &&
        "bar" in o &&
        Array.isArray(o["bar"]) &&
        o["bar"].length === 1 &&
        typeof o["bar"]["0"] === "object" &&
        o["bar"]["0"] != null &&
        Object.keys(o["bar"]["0"]).length === 1 &&
        "foo" in o["bar"]["0"] &&
        typeof o["bar"]["0"]["foo"] === "string"
    );
}

export function isBaz(o: unknown): o is Baz {
    return (
        o != null && typeof o === "object" && Object.keys(o).length === 1 && "foo" in o && typeof o["foo"] === "string"
    );
}
```

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](./LICENSE.md) file for details.
