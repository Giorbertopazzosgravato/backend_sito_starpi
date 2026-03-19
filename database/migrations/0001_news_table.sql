create table news(
    id             serial primary key,
    titolo         varchar(50) NOT NULL,
    descrizione    varchar(300),
    path_immagine  varchar(150),
    data_rilascio  date
);