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

export const UPDATE_LEAGUE = gql`
mutation UpdateLeague(
  $id: ID!
  $description: String
  $public: Boolean
  $password: String
  $maxPlayers: Int
  $state: LeagueState
  $status: LeagueStatus
) {
  updateLeague(
    input:{
      id: $id
 			description: $description
      public: $public
      password: $password
      maxPlayers: $maxPlayers
      state: $state
      status: $status
  }) {
    id
  }
}
`;
