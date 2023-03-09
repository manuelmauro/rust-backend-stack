Feature: Users

  Scenario Outline: Registering a new user
    When we register with username <username> and password <password>
    Then either the user exists or a new user is created

    Examples: 
      | username | password  |
      | alice    | pwd_alice |
      | bob      | pwd_bob   |
      | casey    | pwd_casey |
