// @ts-ignore
export function guard(value: Any): asserts value {
  if (value === undefined) {
    throw new Error("value must be defined");
  }
}

// @ts-ignore
export function assert(value: Any): asserts value {
  if (!value) {
    throw new Error("value must be true");
  }
}
