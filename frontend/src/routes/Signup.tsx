import { gql, useMutation } from "@apollo/client";
import { FormEvent, useState } from "react";
import { useNavigate } from "react-router-dom";

import Form from "../components/Form";

const SIGNUP = gql`
mutation Signup(
  $username: String!
  $email: String!
  $password: String!
) {
  signup(newUser: {
      username: $username
      email: $email
      password: $password
  }) {
    username
  }
}
`;

export default function Signup() {
  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");

  const [error, setError] = useState("");
  const navigate = useNavigate();

  const [signup] = useMutation(SIGNUP, {
    onCompleted: ({_signup}) => {
      navigate('/login');
    },
    onError: ({message}) => {
      setError(message);
    }
  });

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (username !== confirmPassword) {
      setError("Passwords do not match");
      return;
    }

    signup({ variables: {
      username: username,
      email: email,
      password: password,
    }});
  };

  return (
    <Form.FullPage title="Sign Up" submitLabel="Sign Up" error={error} onSubmit={handleSubmit}>
      <Form.Input label="Username" state={setUsername} />
      <Form.Input type="email" label="E-mail" state={setEmail} />
      <Form.Input type="password" label="Password" state={setPassword} />
      <Form.Input type="password" label="Confirm Password" state={setConfirmPassword} />
    </Form.FullPage>
  )
}