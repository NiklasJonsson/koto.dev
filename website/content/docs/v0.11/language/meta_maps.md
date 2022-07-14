+++
title = "Meta Maps"
slug = "meta_maps"
weight = 16
+++

# Meta Maps

Maps can be used to create value types with custom behaviour.

Keys with `@` prefixes go into the map's 'meta map',
which is checked when the map is encountered in operations.

````koto
make_x = |n|
  data: n

  # Overloading the addition operator
  @+: |self, other|
    # A new instance is made with the result of adding the two values together
    make_x self.data + other.data

  # Overloading the subtraction operator
  @-: |self, other|
    make_x self.data - other.data

x1 = make_x 10
x2 = make_x 20

(x1 + x2).data
# -> 30
(x1 - x2).data
# -> -10
````

{% example_playground_link() %}
play.clear_output()

make_x = |n|
  data: n

  # Overloading the addition operator
  @+: |self, other|
    # A new instance is made with the result of adding the two values together
    make_x self.data + other.data

  # Overloading the subtraction operator
  @-: |self, other|
    make_x self.data - other.data

x1 = make_x 10
x2 = make_x 20

print (x1 + x2).data
# -> 30
print (x1 - x2).data
# -> -10

{% end %}
All binary operators can be overloaded following this pattern.

The following meta functions can also be defined:

* `@negate`
  * Overloads the unary negation operator:
    * `@negate: |self| make_x -self.data`
* `@not`
  * Overloads the unary `not` operator:
    * `@not: |self| self.data == 0`
* `@index`
  * Overloads `[]` indexing:
    * `@index: |self, index| self.data + index`
* `@iterator`
  * Customizes how iterators will be made from the map. The function returns an
    iterator that will be used in place of the default iteration behaviour.
    * `@iterator: |self| 0..self.data`
* `@display`
  * Customizes how the map will be displayed when formatted as a string:
    * `@display: |self| "X: {}".format self.data`
* `@type`
  * Provides a String that's used when checking the map's type:
    * `@type: "X"`

todo! Add a note about sharing meta maps between instances.