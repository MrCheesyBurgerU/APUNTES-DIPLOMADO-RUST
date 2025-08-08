// Principios ampliamente utilizaos en la industria para la generación
// de pruebas unitarias

// FAST
// La prueba debe ser rapida. Pruebas lentas no se ejecutan con frecuencia, lo
// cual lleva a errores y poca observabiidad y control de la integracion

// INDEPENDENT
// Las pruebas no pueden depender una de otras, las pruebas se deben poder ejecutar
// de forma independiente y en cualquier orden

// REPEATABLE
// Las pruebas deben ser repetibles en cualquier entorno, si no son repetibles 
// siempre se tiene una excusa de por que fallan

// SELF VALIDATING
// La prueba debe tener una salida booleana, o pasan o falln, no se debe tener
// que leer el archivo de registro para saber si la prueba paso

// TIMELY
// Las pruebas unitarias deben escribirse juso antes del codigo de produccion que
// las hace pasar, es mejor hacer TDD

// TEST DRIVEN DEVELOPMENT
// Se basa en la siguiente idea basica:

// 1. Escribir una prueba que falla
// 2. Escribir el codigo que hace que la prueba pase
// 3. Limpiar o mejorar el codigo mediante refactorizacion

// CICLO NANO
// Consiste en

// 1. Escribe una prueba que falle antes de escribir cualquier codigo real
// 2. No escriba mas pruebas de las necesarias para que falle
// 3. No escriba mas codigo del necesario para que la prueba pase

// CICLO MICRO
// Consiste en red - green - refactor

// 1. Red escribe la preuba que falla
// 2. Green escribe el codigo que la hace pasar
// 3. Refactor limpia el codigo sin romper nada

// CICLO MILICICLO

// A medida que escrbie pruebas mas especificas, el codigo debe volverse mas general.
// Si el codigo no se hace mas general, se queda atascado. 
// Se debe aplicar cuando es muy dificil pasar la siguiente prueba, tiene que escribir
// mucho codigo fuera de los ciclos anteriores

// CICLO PRIMARIO

// Cada cierto tiempo se hace una pausa para ver el panorama completo. Se estan
// respetando los limites correctos? El codigo esta bien organizado? Se esta creando
// una buena arquitectura general?


