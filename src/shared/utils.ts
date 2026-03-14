export function matches<T extends string | number | symbol, R>(
  subject: T,
  cases: Partial<Record<T, () => R>> & { _: (i: T) => R },
): R {
  const handler = cases[subject];
  if (handler) return handler();

  return cases["_"](subject);
}
