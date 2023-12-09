type Input = '1abc2' | 'pqr3stu8vwx' | 'a1b2c3d4e5f' | 'treb7uchet';

type Reverse<T extends string> = T extends `${infer First}${infer Rest}`
  ? `${Reverse<Rest>}${First}`
  : T;

type FirstNumber<T extends string> =
  T extends `${infer N extends number}${infer _}`
    ? N
    : T extends `${infer _}${infer Rest}`
    ? FirstNumber<Rest>
    : never;

type LastNumber<T extends string> = FirstNumber<Reverse<T>>;

type A = `${FirstNumber<'1abc2'>}${LastNumber<'1abc2'>}`;
