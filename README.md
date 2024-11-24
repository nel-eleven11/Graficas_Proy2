# Proyecto 2 - Gráficas por Computadora
## Raycasting

Un proyecto de trazado de rayos en Rust que implementa un diorama con cubos, utilizando materiales con texturas, reflejos, transparencias, y emisiones de luz. Este proyecto incluye funcionalidades avanzadas como una cámara interactiva, materiales personalizables y una fuente de luz dinámica para visualizar escenas tridimensionales.

---

## Tabla de contenido

- Demostración con Gif
- Descripción
- Características
- Estructura del proyecto
- Requisitos previos
- Instalación
- Cómo ejecutar
- Controles


---


## Gif


---

## Descripción

Este proyecto es un raytracer desarrollado en Rust que utiliza cubos como base para construir escenas tridimensionales. Las escenas están representadas como capas apiladas de materiales, cada uno con sus propias características visuales. Este trazador de rayos permite:

- Renderizar escenas con texturas realistas.
- Incorporar materiales con transparencia, reflejos y emisión de luz.
- Interactuar con la escena mediante controles de cámara y cambios dinámicos en la iluminación.
- Crear escenas configurables mediante capas definidas en el archivo diorama.rs.

--- 

## Características

- Soporte para texturas: Carga imágenes como texturas para aplicar a los cubos.
- Materiales avanzados:
- - Transparencia y refracción.
- - Emisión de luz para materiales autoiluminados.
- - Reflectividad configurable.
- Cámara interactiva:
- - Movimiento de rotación y zoom.
- Fuente de luz dinámica:
- - Cambia el color de la luz en tiempo real.
- Diseño modular:
- - Código organizado en módulos para una mejor mantenibilidad.

---

## Requisitos previos

### Dependencias del proyecto
El proyecto utiliza las siguientes dependencias, especificadas en Cargo.toml:

- nalgebra-glm: Librería para álgebra lineal y operaciones matemáticas.
- minifb: Ventana gráfica para renderizar los gráficos.
- image: Carga de texturas desde archivos.
- once_cell: Inicialización perezosa para texturas y mapas normales.

### Herramientas necesarias
Rust: Asegúrate de tener Rust instalado.
- Puedes instalarlo usando Rustup.
- Versión recomendada: 1.70.0 o superior.
Cargo: Herramienta de gestión de proyectos de Rust (viene con Rustup).

---

## Instalación/Ejecución 

Clona este repositorio:
```bash 
git clone https://github.com/tu_usuario/Graficas_Proy2.git
cd Graficas_Proy2
```
Compila el proyecto:
```bash 
cargo build --release
```
Asegúrate de que la carpeta assets/ esté en el mismo directorio que main.rs para que se carguen las texturas correctamente.

Para ejecutar el proyecto:
```bash 
cargo run --release
```
Esto abrirá una ventana gráfica donde se renderiza el diorama tridimensional.

--- 

## Controles

Rotación de la cámara:
* Flechas Izquierda/Derecha: Rotar horizontalmente.
* Flechas Arriba/Abajo: Rotar verticalmente.

Zoom:
* Q: Acercar.
* E: Alejar.

Cambio del color de la luz:
* 1: Luz cálida.
* 2: Luz fría.
* 3: Luz verdosa.
