# Variables:
Las variables son espacios de memoria dónde se almacenan datos, y el nombre de una variable es la referencia a determinado espacio.

## Mutabilidad:
Cuando una variable es inmutable, una vez que un valor está vinculado a un nombre, no se puede
cambiar ese valor.

## Inmutabilidad:
Para hacer que una variable sea mutable hay que escribirla plabara clave:
```Rust
let mut a = 50;
```

# Constantes:
El no poder cambiar el valor de una variable podría haberte recordado otro concepto de
programación que la mayoría de lenguajes tienen: constantes. Al igual que las variables inmutables,
las constantes son valores que están ligados a un nombre y no pueden cambiar, pero hay algunas
diferencias entre las constantes y las variables.

Primero, no está permitido usar mut con constantes. Las constantes no sólo son inmutables por
defecto, sino que siempre lo son.

Las constantes se declaran utilizando la palabra clave const en lugar de la palabra clave let, y el tipo
de valor debe ser anotado. Estamos a punto de cubrir los tipos y las anotaciones en la siguiente
sección, "Tipos de datos", así que no te preocupes por los detalles ahora mismo. Sólo debes saber
que siempre debes anotar el tipo.

Las constantes pueden ser declaradas en cualquier ámbito, incluyendo el ámbito global, lo que las
hace útiles para valores que muchas partes del código necesitan conocer.

La última diferencia es que las constantes pueden ser ajustadas sólo a una expresión constante, no al
resultado de una llamada de función o cualquier otro valor que sólo pueda ser calculado en tiempo
de ejecución.
```Rust
const MAX_POINT_PLAYER = 1000_000_00;
```
# Shadowing:
Los rustaceans dicen que la primera variable es ensombrecida por
la segunda, lo que significa que el valor de la segunda variable es el que aparece cuando se usa la
variable. Podemos sombrear una variable usando el mismo nombre de la variable y repitiendo el uso
de la palabra clave let de la siguiente manera:
```Rust
let _variable_one = 5;
let _variable_one = _variable_one + 20;
let _variable_one = _variable_one * 2;
```

# Tipos de datos:
Cada valor en Rust es de un cierto tipo de datos, lo que le permite saber cómo trabajar con ellos.
Examinaremos dos subconjuntos de tipos de datos: escalar y compuesto.

Ten en cuenta que Rust es un lenguaje de tipo estático lo que significa que debe conocer los tipos de
todas las variables en tiempo de compilación. El compilador normalmente puede inferir qué tipo
queremos usar basándonos en el valor y cómo lo usamos.

## Escalar:
Un tipo escalar representa un valor individual. Rust tiene cuatro tipos de escalares primarios:
enteros, números en coma flotante, Booleans y caracteres. Es posible que los conozcas de otros
lenguajes de programación. Vamos a ver cómo funcionan en Rust.

### Int:
- Un entero es un número sin un componente fraccionario. Usamos un tipo entero en el Capítulo 2, el
tipo u32. Esta declaración de tipo indica que el valor con el que está asociado debe ser un entero sin
signo (los tipos de enteros con signo comienzan con i, en lugar de u) que ocupa 32 bits de espacio.

- Cada variable puede ser con o sin signo y tiene un tamaño explícito. Con o sin signo se refiere a si
es posible que el número sea negativo o positivo, es decir, si el número necesita tener un signo con
él (signed) o si sólo será positivo en algún momento y, por lo tanto, puede representarse sin signo
(unsigned). Es como escribir números en papel: cuando el signo importa, un número se muestra con
un signo más o un signo menos; sin embargo, cuando es seguro asumir que el número es positivo,
se muestra sin signo. Los números con signo se almacenan usando la representación del
complemento dos.

- Cada variable con signo puede almacenar números de -(2n - 1) a 2n - 1 - 1 inclusive, donde n es el
número de bits que utiliza la variante. Así que un i8 puede almacenar números de -(27
) a 27 - 1, lo que equivale de -128 a 127. Las variables sin signo pueden almacenar números de 0 a 2n - 1, por lo que un u8 puede almacenar números de 0 a 28 - 1, lo que equivale de 0 a 255.

- Además, los tipos isize y usize dependen del tipo de ordenador en el que se ejecuta el programa: 64
bits si está en una arquitectura de 64 bits y 32 bits si está en una arquitectura de 32 bits.

### Floating:
- Rust también tiene dos tipos de números en coma flotante, que son números con puntos decimales.
Los tipos de punto flotante de Rust son f32 y f64, que tienen un tamaño de 32 y 64 bits
respectivamente. El tipo por defecto es f64 porque en las CPUs modernas tiene aproximadamente la
misma velocidad que f32 pero es capaz de ser más preciso.

## Operaciones aritméticas
Rust soporta las operaciones matemáticas básicas que cabría esperar para todos los tipos de
números: suma, resta, multiplicación, división y resto. 

## Bool:
Como en la mayoría de los otros lenguajes de programación, un tipo booleano en Rust tiene dos
valores posibles: verdadero y falso.

## Carácter:
Hasta ahora sólo hemos trabajado con números, pero Rust también soporta caracteres. El tipo
carácter de Rust es el tipo alfabético, y el siguiente código muestra una manera de usarlo. (Ten en
cuenta que el literal char se especifica con comillas simples, a diferencia de los literales de cadena
que utilizan comillas dobles.

# Tuplas:
Una tupla es una forma general de agrupar algún número de otros valores con una variedad de tipos
en un tipo compuesto. Las tuplas tienen una longitud fija: una vez declaradas, no pueden crecer o reducirse en tamaño.
```Rust
let _my_tup: (i32, f64, u8) = (50, 5.1, 2);
```

# Arrays:
- Otra forma de tener una colección de valores múltiples es con un array. A diferencia de una tupla, cada elemento de un array debe tener el mismo tipo. Los arrays en Rust son diferentes de los arrays en otros lenguajes ya que en Rust tienen una longitud fija, como las tuplas.

- Los arrays son útiles cuando se quiere que los datos se asignen a la pila en lugar de al montículo(heap) (discutiremos más sobre la pila y el montículo en el Capítulo 4), o cuando queremos asegurarnos de tener siempre un número fijo de elementos. Sin embargo, un array no es tan flexible como el tipo vector. Un vector es un tipo de colección similar proporcionado por la biblioteca estándar que puede crecer o reducirse en tamaño. Si no estás seguro de si usar un array o un vector, probablemente deberías usar un vector.
```Rust
let _array_one = [1, 2, 3, 4, 5];
let _array_two : [i32; 4] = [1, 2, 3, 4];
```

# Function:
La función main, que es el punto de entrada de muchos programas.
También has visto la palabra clave fn, que te permite declarar nuevas funciones.
El código Rust utiliza el snake case como el estilo convencional para los nombres de funciones y
variables. En el snake case, todas las letras son minúsculas y se utiliza el subrayado como conector
de palabras separadas. Aquí hay un programa que contiene un ejemplo de definición de función:
```Rust
fn another_function() {
 println!("Another function.");
}
```