import { gql, useApolloClient, useMutation } from "@apollo/client"
import { LocalState } from "@apollo/client/core/LocalState";
import { FormEvent, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useSetRecoilState } from "recoil";
import { selectedLeagueState } from "../components/App";

import Form from '../components/Form';
import { SELECTED_LEAGUE } from "../constant";

const CREATE_LEAGUE = gql`
mutation CreateLeague (
  $name: String!
  $description: String!
  $public: Boolean!
  $password: String
  $maxPlayers: Int!
  $manualState: Boolean!
) {
  createLeague(input: {
      name: $name
    description: $description
      public: $public
      maxPlayers: $maxPlayers
      password: $password
    manualState: $manualState
  }) {
      id
      name
  }
}
`;

export default function CreateLeague() {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [forPublic, setForPublic] = useState(false);
  const [password, setPassword] = useState("");
  const [maxPlayers, setMaxPlayers] = useState(16);
  const [manualState, setManualState] = useState(false);

  const [ error, setError ] = useState("");

  const navigate = useNavigate();
  const setSelectedLeague = useSetRecoilState(selectedLeagueState);
  const client = useApolloClient();

  const [create] = useMutation(CREATE_LEAGUE, {
    variables: {
      name: name,
      description: description,
      public: forPublic,
      password: password,
      maxPlayers: maxPlayers,
      manualState: manualState,
    },
    onCompleted: ({createLeague}) => {
      setSelectedLeague(createLeague.id);
      localStorage.setItem(SELECTED_LEAGUE, createLeague.id);
      client.resetStore();
      navigate(`/league/${createLeague.id}`);
    },
    onError: ({message}) => {
      setError(message)
    }
  })

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    create();
  };

  return (
    <Form.FullPage title="Create League" submitLabel="Create League" error={error} onSubmit={handleSubmit}>
      <Form.Input label="Name" state={setName} />
      <Form.Input label="Description" state={setDescription} />
      <Form.Number label="Max Players" min={2} max={64} default={maxPlayers} state={setMaxPlayers} />
      <Form.Checkbox label="Manual State" state={setManualState} />
      <div className="flex">
        <Form.Checkbox label="Public" state={setForPublic} />
        <Form.Input label="Password" state={setPassword} disabled={forPublic}/>
      </div>
    </Form.FullPage>
  )
}