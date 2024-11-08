use std::io::{self, Write};

// Estructura para representar una pregunta
struct Question<'a> {
    text: &'a str,
    options: [&'a str; 4],
    answer: char,
}

fn main() {
    // Crear las preguntas
    let questions = vec![
        Question {
            text: "¿Cuál es el operador en Rust para hacer referencia a una variable sin transferir su propiedad (ownership)?",
            options: ["A) *", "B) ref", "C) &", "D) @"],
            answer: 'C',
        },
        Question {
            text: "¿Qué hace el siguiente código de Rust?\n\n\
            let mut x = 10;\nlet y = &mut x;\n*y += 5;\n",
            options: [
                "A) Declara una variable `x` y luego intenta reasignarla a un valor constante.",
                "B) Cambia el valor de `x` a `15` usando una referencia mutable.",
                "C) Causa un error de compilación debido a la mutabilidad.",
                "D) No cambia el valor de `x`."
            ],
            answer: 'B',
        },
        Question {
            text: "¿Cuál de las siguientes opciones describe correctamente el uso de `match` en Rust?",
            options: [
                "A) Solo se puede usar con tipos de datos numéricos.",
                "B) Es una estructura similar a `switch` que permite manejar patrones de forma exhaustiva.",
                "C) No permite devolver valores.",
                "D) Solo se usa con el tipo de datos `Option`."
            ],
            answer: 'B',
        },
        Question {
            text: "En Rust, ¿qué significa que una función sea genérica y cómo se define?",
            options: [
                "A) Que puede usarse con cualquier tipo de dato, y se define con `fn<T>()`.",
                "B) Que solo funciona con números y se define con `fn(num)`.",
                "C) Que acepta tipos específicos y se define con `fn<S>()`.",
                "D) Que solo funciona con cadenas y se define con `fn<String>()`."
            ],
            answer: 'A',
        },
        Question {
            text: "Observa el siguiente código. ¿Cuál será la salida si se ejecuta?\n\n\
            fn main() {\n\
            let v = vec![1, 2, 3];\n\
            let sum: i32 = v.iter().map(|x| x * 2).sum();\n\
            println!(\"{}\", sum);\n\
            }\n",
            options: ["A) Imprime `6`", "B) Imprime `12`", "C) Causa un error en tiempo de compilación.", "D) Imprime `18`"],
            answer: 'B',
        },
        Question {
            text: "¿Cuál de las siguientes afirmaciones sobre el sistema de ownership en Rust es correcta?",
            options: [
                "A) Un valor puede tener múltiples dueños al mismo tiempo.",
                "B) Cuando un valor es movido, su propietario original mantiene acceso a él.",
                "C) Rust permite referencias mutables e inmutables simultáneamente al mismo valor.",
                "D)  Cuando un valor sale de su scope, se libera automáticamente."
            ],
            answer: 'D',
        },
        Question {
            text: "¿Cuál es la función principal del operador ? en Rust?",
            options: [
                "A) Realizar una operación de división segura entre dos enteros.",
                "B) Propagar errores en funciones que devuelven Result, simplificando el manejo de errores.",
                "C) Forzar la conversión entre tipos de datos incompatibles.",
                "D) Cambiar el tipo de una variable a Option."
            ],
            answer: 'B',
        },
        Question {
            text: "¿Cuál es la diferencia entre String y &str en Rust?",
            options: [
                "A) String es una cadena con tamaño fijo, mientras que &str tiene tamaño dinámico.",
                "B) &str siempre apunta a una cadena en el heap, mientras que String apunta a una cadena en el stack.",
                "C) String es un tipo de cadena mutable y asignado en el heap, mientras que &str es una referencia a una cadena inmutable, generalmente en el stack o en constantes.",
                "D) &str permite modificación de su contenido, mientras que String no."
            ],
            answer: 'C',
        },
        
    ];

    loop {
        let mut score = 0;
        let mut correct_answers = 0;

        // Iterar sobre las preguntas
        for (i, question) in questions.iter().enumerate() {
            println!("Pregunta {}: {}", i + 1, question.text);
            for option in question.options.iter() {
                println!("{}", option);
            }
            print!("Tu respuesta: ");
            io::stdout().flush().unwrap();

            // Leer la respuesta del usuario
            let mut user_answer = String::new();
            io::stdin().read_line(&mut user_answer).unwrap();
            let user_answer = user_answer.trim().chars().next().unwrap_or(' ');

            // Comparar la respuesta del usuario con la correcta
            // to_ascii_uppercase() convei
            if user_answer == question.answer {
                println!("¡Correcto!\n");
                score += 50; // Sumar 50 puntos
                correct_answers += 1;
                println!("Puntaje actual: {}", score);
            } else {
                println!("Incorrecto. La respuesta correcta era: {}\n", question.answer);
                println!("Perdiste. Tu puntaje vuelve a cero.\n");
                break;
            }
        }

        // Verificar si contestó todas correctamente
        // len() metodo que devuelve el numero total de de elemento en el vector
        if correct_answers == questions.len() {
            println!("¡Felicidades! Has respondido correctamente a todas las preguntas.");
        }
        println!("Puntaje final: {}\n", score);
        
        println!("¿Quieres intentar de nuevo? (s/n): ");
        let mut retry = String::new();
        io::stdin().read_line(&mut retry).unwrap();
        if retry.trim().to_lowercase() != "s" {
            break;
        }
    }
}
