Feature: Users

  Scenario Outline: Registering a new user
    When we register with username <username>, email <email>, and password <password>
    Then either the user exists or a new user is created

    Examples: 
      | username | email             | password  |
      | alice    | alice@example.com | pwd_alice |
      | bob      | bob@example.com   | pwd_bob   |
      | casey    | casey@example.com | pwd_casey |

  Scenario Outline: Login with a new user
    When we register with username <username>, email <email>, and password <password>
    Then either the user exists or a new user is created
    When we login with email <email> and password <password>
    Then we receive an authorization token

    Examples: 
      | username | email             | password  |
      | alice    | alice@example.com | pwd_alice |
      | bob      | bob@example.com   | pwd_bob   |
      | casey    | casey@example.com | pwd_casey |
