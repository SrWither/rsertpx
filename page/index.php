<!DOCTYPE html>
<html lang="en">

<head>
  <meta charset="UTF-8" />
  <meta http-equiv="X-UA-Compatible" content="IE=edge" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />

  <!-- Compiled and minified CSS -->
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@materializecss/materialize@1.1.0/dist/css/materialize.min.css" />
  <link href="https://fonts.googleapis.com/icon?family=Material+Icons" rel="stylesheet">

  <!-- Compiled and minified JavaScript -->
  <script src="https://cdn.jsdelivr.net/npm/@materializecss/materialize@1.1.0/dist/js/materialize.min.js"></script>

  <title>Index</title>
</head>

<body>
  <header>
    <?php include '_navbar.php' ?>
  </header>

  <main>
    <div class="container">
      <h1><?php echo "Hello World From php" ?></h1>
      <ul class="collapsible">
        <li>
          <div class="collapsible-header">
            <i class="material-icons">filter_drama</i>First
          </div>
          <div class="collapsible-body">
            <span>Lorem ipsum dolor sit amet.</span>
          </div>
        </li>
        <li>
          <div class="collapsible-header">
            <i class="material-icons">place</i>Second
          </div>
          <div class="collapsible-body">
            <span>Lorem ipsum dolor sit amet.</span>
          </div>
        </li>
        <li>
          <div class="collapsible-header">
            <i class="material-icons">whatshot</i>Third
          </div>
          <div class="collapsible-body">
            <span>Lorem ipsum dolor sit amet.</span>
          </div>
        </li>
      </ul>
    </div>
  </main>

  <script>
    M.AutoInit();
  </script>
</body>

</html>