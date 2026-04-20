# Guía de Desarrollo de Plugins para Omigdex

Esta guía explica cómo crear plugins en Lua para Omigdex, una aplicación de descarga de videos. Los plugins permiten extender la funcionalidad de la aplicación conectándose a eventos de descarga y accediendo a la funcionalidad del backend.

## Tabla de Contenidos

- [Resumen](#resumen)
- [Estructura del Plugin](#estructura-del-plugin)
- [Funciones de API Disponibles](#funciones-de-api-disponibles)
- [Ganchos de Eventos](#ganchos-de-eventos)
- [Crear Tu Primer Plugin](#crear-tu-primer-plugin)
- [Ejemplos de Plugins](#ejemplos-de-plugins)
- [Mejores Prácticas](#mejores-prácticas)
- [Prueba de Plugins](#prueba-de-plugins)

## Resumen

Omigdex utiliza un sistema de plugins basado en Lua que se ejecuta en el backend (Rust). Los plugins se escriben en Lua 5.4 y pueden:

- Conectarse a eventos de descarga (inicio, completado, error)
- Acceder a la cola de descargas y al historial
- Leer y escribir archivos
- Registrar mensajes en la consola
- Extender la funcionalidad de la aplicación

Los plugins se cargan desde el directorio `./plugins` en la carpeta de la aplicación. Cada plugin debe ser un archivo `.lua`.

## Estructura del Plugin

Un plugin básico tiene la siguiente estructura:

```lua
-- Inicialización del plugin (opcional)
function init()
    -- Se llama cuando se carga el plugin
end

-- Ganchos de eventos (opcionales)
function on_download_start(data)
    -- Se llama cuando inicia una descarga
end

function on_download_complete(data)
    -- Se llama cuando se completa una descarga
end

function on_download_error(data)
    -- Se llama cuando falla una descarga
end

-- Funciones personalizadas (opcionales)
function mi_funcion_personalizada()
    -- Tu lógica personalizada
end
```

## Funciones de API Disponibles

La tabla `omigdex` proporciona las siguientes funciones:

### Registro

- `omigdex.log(mensaje: string)` - Registrar un mensaje en la consola

  ```lua
  omigdex.log("¡Hola desde el plugin!")
  ```

### Información de Descarga

- `omigdex.get_video_info(url: string) -> table` - Obtener información del video desde una URL

  ```lua
  local info = omigdex.get_video_info("https://youtube.com/watch?v=...")
  -- Retorna: { title: string, platform: string }
  ```

### Gestión de Cola

- `omigdex.get_queue_status() -> table` - Obtener el estado actual de la cola de descargas

  ```lua
  local status = omigdex.get_queue_status()
  -- Retorna: { active_downloads: number, pending_downloads: number, total_downloads: number }
  ```

### Historial

- `omigdex.get_history() -> table` - Obtener el historial de descargas

  ```lua
  local history = omigdex.get_history()
  -- Retorna un array de entradas de descargas
  for i, download in ipairs(history) do
      print(download.title)
      print(download.platform)
      print(download.format)
      print(download.status)
  end
  ```

### Operaciones de Archivos

- `omigdex.read_file(ruta: string) -> string` - Leer el contenido de un archivo

  ```lua
  local content = omigdex.read_file("config.txt")
  ```

- `omigdex.write_file(ruta: string, contenido: string)` - Escribir contenido en un archivo

  ```lua
  omigdex.write_file("output.txt", "¡Hola, Mundo!")
  ```

## Ganchos de Eventos

Los plugins pueden conectarse a los siguientes eventos:

### `on_download_start(data)`

Se llama cuando inicia una descarga.

- **Parámetros**: `data` (string) - Cadena JSON que contiene información de la descarga
- **Ejemplo**:

  ```lua
  function on_download_start(data)
      omigdex.log("Descarga iniciada: " .. data)
  end
  ```

### `on_download_complete(data)`

Se llama cuando una descarga se completa exitosamente.

- **Parámetros**: `data` (string) - Cadena JSON que contiene información de la descarga
- **Ejemplo**:

  ```lua
  function on_download_complete(data)
      omigdex.log("Descarga completada: " .. data)
      -- Registrar en archivo
      omigdex.write_file("completados.log", data .. "\n", "a")
  end
  ```

### `on_download_error(data)`

Se llama cuando falla una descarga.

- **Parámetros**: `data` (string) - Cadena JSON que contiene información de la descarga incluyendo detalles del error
- **Ejemplo**:

  ```lua
  function on_download_error(data)
      omigdex.log("Descarga fallida: " .. data)
      -- Enviar notificación o registrar error
  end
  ```

## Crear Tu Primer Plugin

### Paso 1: Crear el Archivo del Plugin

Crea un nuevo archivo `.lua` en el directorio `plugins`:

```bash
# En el directorio de la aplicación
mkdir plugins
# Crea tu plugin
echo "" > plugins/mi_plugin.lua
```

### Paso 2: Escribir Tu Plugin

Edita el archivo del plugin con tu código:

```lua
-- mi_plugin.lua

function init()
    omigdex.log("¡Mi plugin cargado!")
end

function on_download_complete(data)
    omigdex.log("¡Descarga finalizada exitosamente!")
    
    -- Ejemplo: Guardar info de descarga en un log personalizado
    local log_entry = os.date("%Y-%m-%d %H:%M:%S") .. " - " .. data .. "\n"
    local current_log = omigdex.read_file("mis_descargas.log") or ""
    omigdex.write_file("mis_descargas.log", current_log .. log_entry)
end
```

### Paso 3: Recargar Plugins

Usa el comando de Tauri para recargar plugins:

```javascript
// Desde el frontend
await invoke('reload_plugins');
```

### Paso 4: Habilitar Tu Plugin

```javascript
// Habilitar el plugin
await invoke('enable_plugin', { name: 'mi_plugin' });
```

## Ejemplos de Plugins

### Ejemplo 1: Registrador de Descargas

Registra todas las descargas en un archivo con marcas de tiempo:

```lua
-- registrador_descargas.lua

function on_download_complete(data)
    local timestamp = os.date("%Y-%m-%d %H:%M:%S")
    local log_entry = "[" .. timestamp .. "] " .. data .. "\n"
    
    local existing_log = omigdex.read_file("descargas.log") or ""
    omigdex.write_file("descargas.log", existing_log .. log_entry)
    
    omigdex.log("Descarga registrada en descargas.log")
end
```

### Ejemplo 2: Estadísticas de Plataforma

Rastrea estadísticas de descargas por plataforma:

```lua
-- estadisticas_plataforma.lua

local stats = {}

function init()
    -- Cargar estadísticas existentes
    local stats_file = omigdex.read_file("estadisticas_plataforma.json") or "{}"
    stats = stats_file
    omigdex.log("Plugin de estadísticas de plataforma inicializado")
end

function on_download_complete(data)
    -- Analizar los datos (simplificado - en implementación real usar librería JSON)
    -- Por ahora, solo registrar la finalización
    omigdex.log("Descarga completada - actualizar estadísticas")
end
```

### Ejemplo 3: Validador de URL

Valida URLs antes de la descarga:

```lua
-- validador_url.lua

function validate_download(url)
    local info = omigdex.get_video_info(url)
    if info then
        omigdex.log("URL válida - Título: " .. info.title)
        return true
    else
        omigdex.log("URL inválida o video no encontrado")
        return false
    end
end
```

## Mejores Prácticas

1. **Manejo de Errores**: Siempre maneja errores potenciales en tus plugins

   ```lua
   local success, err = pcall(function()
       omigdex.write_file("output.txt", contenido)
   end)
   
   if not success then
       omigdex.log("Error al escribir archivo: " .. err)
   end
   ```

2. **Rendimiento**: Mantén la lógica del plugin ligera para evitar ralentizar las descargas

3. **Registro**: Usa `omigdex.log()` para depuración y monitoreo

4. **Rutas de Archivos**: Usa rutas relativas cuando sea posible para portabilidad

5. **Nomenclatura**: Usa nombres descriptivos para plugins (ej. `registrador_descargas.lua` en lugar de `plugin1.lua`)

6. **Documentación**: Comenta el código de tu plugin para mantenibilidad

## Prueba de Plugins

### Prueba Manual

1. Coloca tu plugin en el directorio `plugins`
2. Recarga los plugins usando el comando de Tauri
3. Inicia una descarga y verifica los logs de la consola
4. Verifica el comportamiento del plugin

### Depuración

Usa `omigdex.log()` para depurar tu plugin:

```lua
function init()
    omigdex.log("Inicio de inicialización del plugin")
    -- Tu código
    omigdex.log("Inicialización del plugin completada")
end
```

### Problemas Comunes

- **Plugin no carga**: Verifica que la extensión del archivo sea `.lua`
- **Errores de sintaxis**: Verifica la sintaxis de Lua usando un linter de Lua
- **Errores de permiso**: Asegúrate de que la aplicación tenga permisos de escritura en el directorio de plugins

## Comandos de Gestión de Plugins

Los siguientes comandos de Tauri están disponibles para la gestión de plugins:

- `get_plugins()` - Obtener lista de plugins cargados
- `reload_plugins()` - Recargar todos los plugins desde el directorio de plugins
- `enable_plugin(nombre)` - Habilitar un plugin específico
- `disable_plugin(nombre)` - Deshabilitar un plugin específico

## Temas Avanzados

### Análisis de JSON

Para manejo complejo de datos, es posible que necesites analizar JSON. Dado que Lua no tiene soporte integrado para JSON, puedes:

1. Usar manipulación simple de cadenas para necesidades básicas
2. Incluir una librería JSON en tu plugin (si se soporta en versiones futuras)

### Operaciones Asíncronas

Actualmente, los plugins se ejecutan de forma síncrona. Las operaciones de larga duración deben mantenerse al mínimo para evitar bloquear la aplicación.

## Soporte

Para problemas o preguntas sobre el desarrollo de plugins:
- Revisa el plugin de ejemplo en `plugins/example_plugin.lua`
- Consulta la documentación de Omigdex
- Abre un issue en el repositorio de GitHub
