import { gql, useApolloClient, useMutation, useQuery } from "@apollo/client";
import { FormEvent, useState } from "react";
import { useParams } from "react-router-dom";
import { MY_ID } from "../constant";
import {MY_TEAM_FOR_LEAGUE} from "../graphql/Team";
import { Card, CardBody, CardHeader, CardHeaderLink, CardSubHeader } from "./Card";
import Form from "./Form";
import Loader from "./Loader";
import Roster from "./Roster";

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

  let playerCount = 0

  const players = team.players.map((player: any) => {
    playerCount++;
    return (
      <tr key={player.id} className="px-2">{player.name}</tr>
    )
  })

  console.log(team);

  return (
    <div>
      <Card>
        <CardHeaderLink title={"Team: " + team.name} to={`/team/${team.id}`} />
        <CardBody>
          Gold: {team.gold}<br/>
          <table>
            <thead>
              <th className="text-left">Players: {playerCount}</th>
            </thead>
            <tbody>
              <Roster roster={team.roster} />
            </tbody>
          </table>
        </CardBody>
      </Card>
    </div>
  )
}
