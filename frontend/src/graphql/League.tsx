import { gql } from "@apollo/client";

export const GET_LEAGUE = gql`
query GetLeague (
	$id: ID!
){
  league(id: $id) {
    id
    name
    description
    public
    maxPlayers
    state
    manualState
    owner {
      id
      username
    }
    managers {
      id
      username
    }
    teams {
      id
      name
      owner {
        id
        username
      }
    }
  }
}
`;

export const JOIN_LEAGUE = gql`
mutation JoinLeague(
  $id: ID!
) {
  joinLeague(id:$id) {
      id
  }
}
`;