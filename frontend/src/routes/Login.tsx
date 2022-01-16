import { gql, useMutation } from "@apollo/client";
import { FormEvent, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useSetRecoilState } from "recoil";
import { tokenState } from "../components/App";

import Form from "../components/Form";
import { AUTH_TOKEN } from "../constant";

export const LOGIN = gql`
mutation Login(
  $usernameOrEmail: String!
  $password: String!
) {
  login(credentials: {
      usernameOrEmail: $usernameOrEmail
      password: $password
  }) {
    id
      username
      email
      role
      token
  }
}
`;

export default function Login() {
  const navigate = useNavigate();
  const setToken = useSetRecoilState(tokenState);

  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  const [error, setError] = useState("");

  const [login] = useMutation(LOGIN, {
    variables: {
      usernameOrEmail: username,
      password: password,
    },
    onCompleted: ({ login }) => {
      localStorage.setItem(AUTH_TOKEN, login.token);
      setToken(login.token);
      navigate('/');
    },
    onError: ({message}) => {
      setError(message);
    }
  })

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    login();
  };

  return (
    <Form.FullPage title="Login" submitLabel="Login" error={error} onSubmit={handleSubmit}>
      <Form.Input type="input" label="Username" state={setUsername} />
      <Form.Input type="password" label="Password" state={setPassword} />
    </Form.FullPage>
  )
}