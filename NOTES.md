What LEAGUES have been created by USER      LEAGUES->USER (Done)
What LEAGUES is a USER a member of          LEAGUES->USER
Which USERS are in a LEAGUE                 USERS->LEAGUE
Which TEAMS are in a LEAGUE                 TEAMS->LEAGUE
Which PLAYERS are in a TEAM                 PLAYERS->TEAM
Which PLAYERS are in a LEAGUE               PLAYERS->LEAGUE


type Query {
    users: [User!]!
    user(id: ID!): User!
    leagues: [League!]!
    league(id: ID!): League!
    teams: [Team!]!
    team(id: ID!): Team!
    players: [Player!]
    player(id: ID!): Player!
}

type User {
    id: ID!
    username: String!
    leagues: [League!]!
    teams: [Team!]!
}

type League {
    id: ID!
    name: String!
    owner: User!
    users: [User!]!
    teams: [Team!]!
}

type Team {
    id: ID!
    name: String!
    owner: User!
    league: League!
    players: [Players!]!
}

type Player {
    id: ID!
    name: String!
    league: League!
    team: Team!
}