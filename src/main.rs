use rusqlite::{Connection, Result};
use std::collections::HashMap;
use web_view::*;

struct Producto {
    nombre: String,
    descripcion: String,
    precio: f64,
    cantidad_disponible: f64,
    cantidad_minima: f64,
}

// Implementación de métodos para Producto
impl Producto {
    fn new(
        nombre: String,
        descripcion: String,
        precio: f64,
        cantidad_disponible: f64,
        cantidad_minima: f64,
    ) -> Producto {
        Producto {
            nombre,
            descripcion,
            precio,
            cantidad_disponible,
            cantidad_minima,
        }
    }

    fn mostrar_detalle(&self) {
        println!("Nombre: {}", self.nombre);
        println!("Descripción: {}", self.descripcion);
        println!("Precio: ${}", self.precio);
        println!("Cantidad Disponible: {}", self.cantidad_disponible);
    }
}

struct GestorProductos {
    productos: HashMap<String, Producto>,
}

// Implementación de métodos para GestorProductos
impl GestorProductos {
    fn new() -> GestorProductos {
        GestorProductos {
            productos: HashMap::new(),
        }
    }

    fn agregar_producto(&mut self, nombre: String, producto: Producto) {
        self.productos.insert(nombre, producto);
    }

    fn buscar_producto(&self, nombre: &str) -> Option<&Producto> {
        self.productos.get(nombre)
    }

    fn comprar_producto(&mut self, nombre: &str, cantidad: f64) -> Result<(), String> {
        if let Some(producto) = self.productos.get_mut(nombre) {
            producto.cantidad_disponible += cantidad;
            Ok(())
        } else {
            Err("Producto no encontrado".to_string())
        }
    }

    fn vender_producto(&mut self, nombre: &str, cantidad: f64) -> Result<(), String> {
        if let Some(producto) = self.productos.get_mut(nombre) {
            if producto.cantidad_disponible >= cantidad {
                producto.cantidad_disponible -= cantidad;
                Ok(())
            } else {
                Err("Cantidad insuficiente en el inventario".to_string())
            }
        } else {
            Err("Producto no encontrado".to_string())
        }
    }

    fn verificar_cantidad_minima(&self, nombre: &str) -> bool {
        if let Some(producto) = self.productos.get(nombre) {
            producto.cantidad_disponible <= producto.cantidad_minima
        } else {
            false
        }
    }
}

struct GestorUsuarios {
    conn: Connection,
}

// Implementación de métodos para GestorUsuarios
impl GestorUsuarios {
    fn new(conn: Connection) -> GestorUsuarios {
        GestorUsuarios { conn }
    }

    fn gestionar_cuenta(gestor_usuarios: &GestorUsuarios, usuario_actual: &str) {
        loop {
            limpiar_consola();
            println!("Gestionar Cuenta");
            println!("----------------");
            println!("1. Cambiar Nombre");
            println!("2. Cambiar Correo Electrónico");
            println!("3. Cambiar Contraseña");
            println!("4. Volver al Menú Principal");

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Error al leer la entrada");

            match input.trim() {
                "1" => {
                    println!("Ingrese su nuevo nombre:");
                    let mut nuevo_nombre = String::new();
                    std::io::stdin()
                        .read_line(&mut nuevo_nombre)
                        .expect("Error al leer la entrada");
                    let nuevo_nombre = nuevo_nombre.trim();

                    match gestor_usuarios.cambiar_nombre(usuario_actual, nuevo_nombre) {
                        Ok(()) => println!("Nombre cambiado exitosamente"),
                        Err(err) => println!("Error al cambiar el nombre: {}", err),
                    }
                }
                "2" => {
                    println!("Ingrese su nuevo correo electrónico:");
                    let mut nuevo_correo = String::new();
                    std::io::stdin()
                        .read_line(&mut nuevo_correo)
                        .expect("Error al leer la entrada");
                    let nuevo_correo = nuevo_correo.trim();

                    match gestor_usuarios.cambiar_correo(usuario_actual, nuevo_correo) {
                        Ok(()) => println!("Correo electrónico cambiado exitosamente"),
                        Err(err) => println!("Error al cambiar el correo electrónico: {}", err),
                    }
                }
                "3" => {
                    println!("Ingrese su nueva contraseña:");
                    let mut nueva_contrasena = String::new();
                    std::io::stdin()
                        .read_line(&mut nueva_contrasena)
                        .expect("Error al leer la entrada");
                    let nueva_contrasena = nueva_contrasena.trim();

                    match gestor_usuarios.cambiar_contrasena(usuario_actual, nueva_contrasena) {
                        Ok(()) => println!("Contraseña cambiada exitosamente"),
                        Err(err) => println!("Error al cambiar la contraseña: {}", err),
                    }
                }
                "4" => {
                    break;
                }
                _ => {
                    println!("Opción no válida");
                }
            }
            println!("Presione Enter para continuar...");
            let mut _pause = String::new();
            std::io::stdin()
                .read_line(&mut _pause)
                .expect("Error al leer la entrada");
        }
    }

    fn crear_usuario(
        &self,
        nombre: &str,
        email: &str,
        contrasena: &str,
        es_administrador: i32,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT INTO usuarios (nombre, email, contrasena, es_administrador) VALUES (?1, ?2, ?3, ?4)",
            &[nombre, email, contrasena, es_administrador.to_string().as_str()],
        )?;
        Ok(())
    }

    fn eliminar_usuario(&self, email: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM usuarios WHERE email = ?1", &[email])?;
        Ok(())
    }

    fn autenticar_usuario(&self, email: &str, contrasena: &str) -> Result<bool> {
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM usuarios WHERE email = ?1 AND contrasena = ?2")?;
        let count: i64 = stmt.query_row(&[email, contrasena], |row| row.get(0))?;
        Ok(count > 0)
    }

    fn cambiar_contrasena(&self, email: &str, nueva_contrasena: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE usuarios SET contrasena = ?1 WHERE email = ?2",
            &[nueva_contrasena, email],
        )?;
        Ok(())
    }

    fn cambiar_nombre(&self, email: &str, nuevo_nombre: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE usuarios SET nombre = ?1 WHERE email = ?2",
            &[nuevo_nombre, email],
        )?;
        Ok(())
    }

    fn cambiar_correo(&self, email: &str, nuevo_email: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE usuarios SET email = ?1 WHERE email = ?2",
            &[nuevo_email, email],
        )?;
        Ok(())
    }

    fn es_administrador(&self, email: &str) -> bool {
        let mut stmt = self
            .conn
            .prepare("SELECT es_administrador FROM usuarios WHERE email = ?1")
            .expect("Error al preparar la consulta");
        let mut rows = stmt.query(&[email]).expect("Error al ejecutar la consulta");

        if let Some(row) = rows.next().expect("Error al obtener la fila") {
            let es_administrador: bool = row.get(0).expect("Error al obtener el valor");
            es_administrador
        } else {
            false
        }
    }
}

fn limpiar_consola() {
    print!("{}[2J", 27 as char);
}

// Función para el inicio de sesión
fn iniciar_sesion(gestor_usuarios: &GestorUsuarios) -> Option<String> {
    println!("Iniciar Sesión");
    println!("--------------");
    println!("Ingrese su correo electrónico:");
    let mut email = String::new();
    std::io::stdin()
        .read_line(&mut email)
        .expect("Error al leer la entrada");
    let email = email.trim();

    println!("Ingrese su contraseña:");
    let mut contrasena = String::new();
    std::io::stdin()
        .read_line(&mut contrasena)
        .expect("Error al leer la entrada");
    let contrasena = contrasena.trim();

    // Verificar las credenciales ingresadas
    if gestor_usuarios
        .autenticar_usuario(email, contrasena)
        .unwrap_or(false)
    {
        Some(email.to_string())
    } else {
        println!("Correo electrónico o contraseña incorrectos");
        None
    }
}

// Función para mostrar la ventana emergente
fn mostrar_ventana_emergente() {
    let html_content = r#"
        <html>
        <head>
            <title>Advertencia de Inventario</title>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    text-align: center;
                    margin: 20px;
                }
                h1 {
                    color: red;
                }
            </style>
        </head>
        <body>
            <h1>Advertencia de Inventario</h1>
            <p>La cantidad de uno o más productos ha alcanzado su cantidad mínima.</p>
        </body>
        </html>
    "#;

    web_view::builder()
        .title("Advertencia de Inventario")
        .content(Content::Html(html_content.to_string()))
        .size(400, 200)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}

// Main que tiene los menus y logica general junto con el match para la seleccion del usuario
fn main() {
    let conn = rusqlite::Connection::open("users.db").expect("Error al abrir la base de datos");
    let mut gestor_productos = GestorProductos::new();
    let gestor_usuarios = GestorUsuarios::new(conn);

    let mut usuario_actual = None;

    while usuario_actual.is_none() {
        usuario_actual = iniciar_sesion(&gestor_usuarios)
    }

    let es_administrador = gestor_usuarios.es_administrador(usuario_actual.as_ref().unwrap());

    loop {
        limpiar_consola();
        println!("Bienvenido a Chepe te Vende");
        println!("1. Agregar Producto");
        println!("2. Consultar Producto");
        println!("3. Comprar Producto");
        println!("4. Vender Producto");
        println!("5. Gestionar Cuenta");

        if es_administrador {
            println!("6. Añadir Usuario");
            println!("7. Eliminar Usuario");
        }

        println!("8. Cerrar sesion");
        println!("9. Salir");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la entrada");

        match input.trim() {
            "1" => {
                limpiar_consola();
                println!("Agregando Producto");
                println!("------------------");
                println!("Ingrese el nombre del producto:");
                let mut nombre = String::new();
                std::io::stdin()
                    .read_line(&mut nombre)
                    .expect("Error al leer la entrada");
                let nombre = nombre.trim().to_string();

                println!("Ingrese la descripción del producto:");
                let mut descripcion = String::new();
                std::io::stdin()
                    .read_line(&mut descripcion)
                    .expect("Error al leer la entrada");
                let descripcion = descripcion.trim().to_string();

                println!("Ingrese el precio del producto:");
                let mut precio = String::new();
                std::io::stdin()
                    .read_line(&mut precio)
                    .expect("Error al leer la entrada");
                let precio: f64 = precio.trim().parse().expect("Error al convertir el precio");

                println!("Ingrese la cantidad disponible del producto:");
                let mut cantidad = String::new();
                std::io::stdin()
                    .read_line(&mut cantidad)
                    .expect("Error al leer la entrada");
                let cantidad: f64 = cantidad
                    .trim()
                    .parse()
                    .expect("Error al convertir la cantidad");

                println!("Ingrese la cantidad mínima que puede haber de {}: ", nombre);

                let mut cantidad_minima = String::new();
                std::io::stdin()
                    .read_line(&mut cantidad_minima)
                    .expect("Error al leer la entrada");
                let cantidad_minima: f64 = cantidad_minima
                    .trim()
                    .parse()
                    .expect("Error al convertir la cantidad mínima");

                gestor_productos.agregar_producto(
                    nombre.clone(),
                    Producto::new(nombre, descripcion, precio, cantidad, cantidad_minima),
                );
                println!("Producto agregado exitosamente!");
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Error al leer la entrada");
            }
            "2" => {
                limpiar_consola();
                println!("Consultando Producto");
                println!("--------------------");
                println!("Ingrese el nombre del producto a consultar:");
                let mut nombre = String::new();
                std::io::stdin()
                    .read_line(&mut nombre)
                    .expect("Error al leer la entrada");
                let nombre = nombre.trim().to_string();

                if let Some(producto) = gestor_productos.buscar_producto(&nombre) {
                    producto.mostrar_detalle();
                } else {
                    println!("Producto no encontrado");
                }
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Error al leer la entrada");
            }
            "3" => {
                limpiar_consola();
                println!("Comprando Producto");
                println!("-------------------");
                println!("Ingrese el nombre del producto a comprar:");
                let mut nombre = String::new();
                std::io::stdin()
                    .read_line(&mut nombre)
                    .expect("Error al leer la entrada");
                let nombre = nombre.trim().to_string();

                println!("Ingrese la cantidad a comprar:");
                let mut cantidad = String::new();
                std::io::stdin()
                    .read_line(&mut cantidad)
                    .expect("Error al leer la entrada");
                let cantidad: f64 = cantidad
                    .trim()
                    .parse()
                    .expect("Error al convertir la cantidad");

                match gestor_productos.comprar_producto(&nombre, cantidad) {
                    Ok(()) => {
                        println!("Compra realizada con éxito!");
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                }
                println!("Presione Enter para continuar...");
                let mut _pause = String::new();
                std::io::stdin()
                    .read_line(&mut _pause)
                    .expect("Error al leer la entrada");
            }

            "4" => {
                limpiar_consola();
                println!("Vendiendo Producto");
                println!("-------------------");
                println!("Ingrese el nombre del producto a vender:");
                let mut nombre = String::new();
                std::io::stdin()
                    .read_line(&mut nombre)
                    .expect("Error al leer la entrada");
                let nombre = nombre.trim().to_string();

                println!("Ingrese la cantidad a vender:");
                let mut cantidad = String::new();
                std::io::stdin()
                    .read_line(&mut cantidad)
                    .expect("Error al leer la entrada");
                let cantidad: f64 = cantidad
                    .trim()
                    .parse()
                    .expect("Error al convertir la cantidad");

                match gestor_productos.vender_producto(&nombre, cantidad) {
                    Ok(()) => {
                        println!("Venta realizada con éxito!");
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                }
                println!("Presione Enter para continuar...");
                let mut _pause = String::new();
                std::io::stdin()
                    .read_line(&mut _pause)
                    .expect("Error al leer la entrada");
            }

            "5" => {
                GestorUsuarios::gestionar_cuenta(
                    &gestor_usuarios,
                    usuario_actual.as_ref().unwrap(),
                );
            }

            "6" => {
                limpiar_consola();
                println!("Añadir Usuario");
                println!("---------------");
                println!("Ingrese el nombre del nuevo usuario:");
                let mut nuevo_nombre = String::new();
                std::io::stdin()
                    .read_line(&mut nuevo_nombre)
                    .expect("Error al leer la entrada");
                let nuevo_nombre = nuevo_nombre.trim().to_string();

                println!("Ingrese el correo electrónico del nuevo usuario:");
                let mut nuevo_correo = String::new();
                std::io::stdin()
                    .read_line(&mut nuevo_correo)
                    .expect("Error al leer la entrada");
                let nuevo_correo = nuevo_correo.trim().to_string();

                println!("Ingrese la contraseña del nuevo usuario:");
                let mut nueva_contrasena = String::new();
                std::io::stdin()
                    .read_line(&mut nueva_contrasena)
                    .expect("Error al leer la entrada");
                let nueva_contrasena = nueva_contrasena.trim().to_string();

                println!("¿El nuevo usuario será administrador? (s/n):");
                let mut es_administrador_input = String::new();
                std::io::stdin()
                    .read_line(&mut es_administrador_input)
                    .expect("Error al leer la entrada");
                let es_administrador = if es_administrador_input.trim().to_lowercase() == "s" {
                    1
                } else {
                    0
                };

                match gestor_usuarios.crear_usuario(
                    &nuevo_nombre,
                    &nuevo_correo,
                    &nueva_contrasena,
                    es_administrador,
                ) {
                    Ok(()) => println!("Usuario agregado exitosamente"),
                    Err(err) => println!("Error al agregar usuario: {}", err),
                }
            }
            "7" => {
                limpiar_consola();
                println!("Ingrese el correo electrónico del usuario a eliminar:");
                let mut email = String::new();
                std::io::stdin()
                    .read_line(&mut email)
                    .expect("Error al leer la entrada");
                let email = email.trim();

                match gestor_usuarios.eliminar_usuario(email) {
                    Ok(()) => println!("Usuario eliminado correctamente"),
                    Err(err) => println!("Error al eliminar el usuario: {}", err),
                }
            }

            "8" => {
                limpiar_consola();
                println!("Sesión cerrada.");
                usuario_actual = None;
                // Solicitar inicio de sesión nuevamente
                while usuario_actual.is_none() {
                    usuario_actual = iniciar_sesion(&gestor_usuarios);
                }
            }

            "9" => {
                println!("Saliendo...");
                break;
            }

            _ => {
                println!("Opción no válida");
            }
        }

        for (nombre, _) in gestor_productos.productos.iter() {
            if gestor_productos.verificar_cantidad_minima(nombre) {
                mostrar_ventana_emergente();
                break; // Detener el bucle después de mostrar una ventana emergente
            }
        }
    }
}
