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
      username
    }
    managers {
      username
    }
    team {
      id
      name
    }
  }
}
`;