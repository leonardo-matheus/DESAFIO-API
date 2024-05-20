<?php
// INIT
$dsn = "sqlite:" . __DIR__ . "/database.sqlite";
$db = new PDO($dsn);

// CFG
$db->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION);
$db->exec("PRAGMA synchronous = OFF;");
$db->exec("PRAGMA journal_mode = OFF;");

// Create Table
function createTable($db) {
    $sql = "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                email TEXT NOT NULL
            )";
    $db->exec($sql);
}

// Create Database and Table If Not Exists
createTable($db);

// Generate Random Data
function generateRandomData($db, $num) {
    $insert = $db->prepare("INSERT INTO users (name, email) VALUES (:name, :email)");
    $db->beginTransaction();
    for ($i = 1; $i <= $num; $i++) {
        $name = "User" . $i;
        $email = "user" . $i . "@example.com";
        $insert->execute([':name' => $name, ':email' => $email]);
    }
    $db->commit();
}


// Simulate $_SERVER variables for CLI testing
if (php_sapi_name() == "cli") {
    if (!isset($argv[1])) {
        echo "Usage: php api.php <endpoint> [request_method]\n";
        exit(1);
    }

    $_SERVER['REQUEST_URI'] = $argv[1];
    $_SERVER['REQUEST_METHOD'] = isset($argv[2]) ? strtoupper($argv[2]) : 'GET';
} 

// Router Handling
$requestMethod = $_SERVER['REQUEST_METHOD'];
$requestUri = parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH);

switch ($requestUri) {
    case '/create':
        if ($requestMethod === 'POST') {
            // Drop Index if Exists
            $db->exec("DROP INDEX IF EXISTS idx_name;");

            // Insert
            $data = generateRandomData($db, 10000);
            $sql = "INSERT INTO users (name, email) VALUES $data;";
            $db->exec($sql);

            // Create Index
            $db->exec("CREATE INDEX IF NOT EXISTS idx_name ON users (name);");

            echo json_encode(["status" => "success", "message" => "10.000 registros criados."]);
        }
        break;

    case '/retrieve':
        if ($requestMethod === 'GET') {
            // Select
            $stmt = $db->query("SELECT id, name, email FROM users LIMIT 10000");
            $result = $stmt->fetchAll(PDO::FETCH_ASSOC);
            echo json_encode($result);
        }
        break;

    case '/clear':
        if ($requestMethod === 'POST') {
            // Clear
            $db->exec("DELETE FROM users");
            $db->exec("VACUUM;"); // Optional for optimization
            echo json_encode(["status" => "success", "message" => "Banco de dados limpo."]);
        }
        break;

    case '/count':
        if ($requestMethod === 'GET') {
            // Count
            $stmt = $db->query("SELECT COUNT(*) as count FROM users");
            $count = $stmt->fetch(PDO::FETCH_ASSOC)['count'];
            echo json_encode(["status" => "success", "count" => $count]);
        }
        break;

    default:
        echo json_encode(["status" => "error", "message" => "Endpoint não encontrado."]);
        break;
}
