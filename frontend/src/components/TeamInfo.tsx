import { gql, useApolloClient, useMutation, useQuery } from "@apollo/client";
import { FormEvent, useState } from "react";
import { useParams } from "react-router-dom";
import { MY_ID } from "../constant";
import { Card, CardBody, CardHeader } from "./Card";
import Form from "./Form";
import Loader from "./Loader";

const CREATE_TEAM = gql`
mutation CreateTeam (
  $name: String!
  $leagueId: ID!
) {
  createTeam(name: $name, leagueId: $leagueId) {
    id
    name
  }
}
`;

function CreateTeamForm() {
  const { id } = useParams();
  const [ name, setName ] = useState("");
  const client = useApolloClient();

  const [ create ] = useMutation(CREATE_TEAM, {
    variables: {
      name: name,
      leagueId: id,
    },
    onCompleted: (data) => {
      client.resetStore();
    },
    onError: ({message}) => {
      setError(message);
    }
  })


  const [ error, setError ] = useState("");

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    create();
  };

  return (
    <Card>
      <CardHeader title="Create Team" />
      <CardBody>
        <Form.Inline submitLabel="Create Team" onSubmit={handleSubmit} error={error}>
          <Form.Input label="Team Name" state={setName}/>
        </Form.Inline>
      </CardBody>
    </Card>
  )
}

const MY_TEAM_FOR_LEAGUE = gql`
query MyTeamForLeague (
  $ownerId: ID!
  $leagueId: ID!
) {
  teams(filter:{
    owner: $ownerId
    league: $leagueId
  }) {
    id
    name
    gold
    owner {
      id
      username
    }
  }
}
`;

export function TeamInfo() {
  const { id } = useParams();
  const my_id = localStorage.getItem(MY_ID);

  const { loading, error, data } = useQuery(MY_TEAM_FOR_LEAGUE, {
    variables: {
      ownerId: my_id,
      leagueId: id,
    }
  });

  if (loading) {
    <Loader />
  }

  if (data) {
    if (data.teams.length === 0) {
      return (
        <CreateTeamForm />
      )
    }
  }

  var team = null

  if (data) {
    team = data.teams[0];
  } else {
    return (
      <Loader />
    )
  }

  if (error) {
    console.error(error);
  }

  return (
    <div>
      <Card>
        <CardHeader title={"Team: " + team.name} />
        <CardBody>
          Gold: {team.gold}<br/>

        </CardBody>
      </Card>
    </div>
  )
}
