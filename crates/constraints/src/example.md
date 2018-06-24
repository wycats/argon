# Constraints

```
<T0> = <T1>
<T1> = {integer}
<T0> = i64
```

---

Applying `<T0> = <T1>`

Solution:

```
{
  <T0>: <T1>
}
```

---

Applying `<T1> = {integer}`

Solution:

```
{
  <T0>: {T1}
}

{
  <T0>: {integer},
  <T1>: {integer}
}
```

---

Applying `<T0> = i64`

Solution:

```
{
  <T0>: {T1}
}

{
  <T0>: i64,
  <T1>: i64 // we remembered the equivalence
}
```
