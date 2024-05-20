<?php
// INIT
$dsn = "sqlite:db/database.sqlite";
$db = new PDO($dsn);

// CFG
$db->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION);
$db->exec("PRAGMA synchronous = OFF;");
$db->exec("PRAGMA journal_mode = OFF;");

// Create
function createTable($db) {
    $sql = "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                email TEXT NOT NULL
            )";
    $db->exec($sql);
}

// Random
function generateRandomData($num) {
    $data = [];
    for ($i = 1; $i <= $num; $i++) {
        $name = "User" . $i;
        $email = "user" . $i . "@example.com";
        $data[] = "('$name', '$email')";
    }
    return implode(', ', $data);
}

// Cria
createTable($db);

// Router
$requestMethod = $_SERVER['REQUEST_METHOD'];
$requestUri = $_SERVER['REQUEST_URI'];

switch ($requestUri) {
    case '/create':
        if ($requestMethod === 'POST') {
            // Opt
            $db->exec("DROP INDEX IF EXISTS idx_name;");
            
            // Insert
            $data = generateRandomData(10000);
            $sql = "INSERT INTO users (name, email) VALUES $data;";
            $db->exec($sql);

            // Index
            $db->exec("CREATE INDEX IF NOT EXISTS idx_name ON users (name);");

            echo json_encode(["status" => "success", "message" => "10.000 registros criados."]);
        }
        break;

    case '/retrieve':
        if ($requestMethod === 'GET') {
            // GET
            $stmt = $db->query("SELECT id, name, email FROM users LIMIT 10000");
            $result = $stmt->fetchAll(PDO::FETCH_ASSOC);
            echo json_encode($result);
        }
        break;

    case '/clear':
        if ($requestMethod === 'POST') {
            // Limpa
            $db->exec("DELETE FROM users");
            $db->exec("VACUUM;"); // Opcional para otimização
            echo json_encode(["status" => "success", "message" => "Banco de dados limpo."]);
        }
        break;

    default:
        echo json_encode(["status" => "error", "message" => "Endpoint não encontrado."]);
        break;
}
?>
