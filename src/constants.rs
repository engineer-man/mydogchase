pub static ARCHIVE_DATA: &[u8] = include_bytes!("chase.zip");
pub static EXAMPLE_CONFIG: &str = r#"ip = "127.0.0.1"
port = 8000
ssl_enabled = false
ssl_port = 8443
ssl_cert_path = "pems/cert.pem"
ssl_key_path = "pems/key.pem"

[routes]
"/" = ["templates/home.html", "public/chase/"]
"/error" = ["templates/error.html"]
"#;
pub static EXAMPLE_HOME: &str = r#"<!doctype html>
<html>
<head>
    <title>Chase The Dog</title>
    <link rel="shortcut icon" href="static/favicon.ico">
    <style>
        body {
            background: #333;
        }
        h1 {
            text-align: center;
            font-size: 72px;
            font-weight: bold;
            color: #fff;
            margin-bottom: 0;
        }
        h3 {
            text-align: center;
            font-size: 32px;
            font-weight: bold;
            color: #fff;
            margin-top: 0;
        }
        h5 {
            text-align: center;
            font-size: 22px;
            font-weight: bold;
            color: #fff;
            margin-top: 10px;
            margin-bottom: 25px;
        }
        .pictures {
            text-align: center;
        }
        .pictures div {
            background-size: cover;
            background-position: center;
            display: inline-block;
            width: 500px;
            height: 500px;
        }
        @media (max-width: 999px) {
            h1 {
                font-size: 48px;
            }
            .pictures div {
                width: 100%;
                height: 500px;
                margin-bottom: 10px;
            }
        }
    </style>
</head>
<body>
    <h1>Chase's Awesome Photo Gallery</h1>
    <h5>Dedicated to the best dog to ever walk the earth.<br />I will cherish every memory of you and I and I will not soon forget about you.<br />I love you more than anything in this world.</h5>
    <h3>Rest In Peace 3/31/2017 - 1/17/2022</h3>
    <div class="pictures">
    </div>
</body>
</html>
"#;
pub static EXAMPLE_ERROR: &str = r#"<!doctype html>
<html lang="en-US">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=yes" />
    <title>guacamole</title>
    <link rel="stylesheet" type="text/css" href="https://thomasf.github.io/solarized-css/solarized-dark.min.css"></link>
    <meta name="viewport" content="width=device-width, initial-scale=1">
</head>
<body>
    <h1>ERROR</h1>
    <p>This page does not exist.</p>
</body>
</html>
"#;
