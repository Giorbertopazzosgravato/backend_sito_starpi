--
-- PostgreSQL database dump
--

\restrict zd3QxTTP8fcpxDce1fc0pvbRCyWleZEKtGXgWTXd92wM8NX0eMTMJg92NPi0GVX

-- Dumped from database version 18.3
-- Dumped by pg_dump version 18.3

-- Started on 2026-03-23 16:54:32

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 219 (class 1259 OID 16530)
-- Name: _sqlx_migrations; Type: TABLE; Schema: public; Owner: giacomo
--

CREATE TABLE public._sqlx_migrations (
    version bigint NOT NULL,
    description text NOT NULL,
    installed_on timestamp with time zone DEFAULT now() NOT NULL,
    success boolean NOT NULL,
    checksum bytea NOT NULL,
    execution_time bigint NOT NULL
);


ALTER TABLE public._sqlx_migrations OWNER TO giacomo;

--
-- TOC entry 220 (class 1259 OID 16542)
-- Name: categoria; Type: TABLE; Schema: public; Owner: giacomo
--

CREATE TABLE public.categoria (
    id integer CONSTRAINT category_id_not_null NOT NULL,
    nome character varying(20)
);


ALTER TABLE public.categoria OWNER TO giacomo;

--
-- TOC entry 221 (class 1259 OID 16546)
-- Name: category_id_seq; Type: SEQUENCE; Schema: public; Owner: giacomo
--

CREATE SEQUENCE public.category_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.category_id_seq OWNER TO giacomo;

--
-- TOC entry 5087 (class 0 OID 0)
-- Dependencies: 221
-- Name: category_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: giacomo
--

ALTER SEQUENCE public.category_id_seq OWNED BY public.categoria.id;


--
-- TOC entry 222 (class 1259 OID 16547)
-- Name: dipartimento; Type: TABLE; Schema: public; Owner: giacomo
--

CREATE TABLE public.dipartimento (
    id integer NOT NULL,
    nome character varying(50)
);


ALTER TABLE public.dipartimento OWNER TO giacomo;

--
-- TOC entry 223 (class 1259 OID 16551)
-- Name: dipartimento_id_seq; Type: SEQUENCE; Schema: public; Owner: giacomo
--

CREATE SEQUENCE public.dipartimento_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.dipartimento_id_seq OWNER TO giacomo;

--
-- TOC entry 5088 (class 0 OID 0)
-- Dependencies: 223
-- Name: dipartimento_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: giacomo
--

ALTER SEQUENCE public.dipartimento_id_seq OWNED BY public.dipartimento.id;


--
-- TOC entry 224 (class 1259 OID 16552)
-- Name: iscrizione; Type: TABLE; Schema: public; Owner: giacomo
--

CREATE TABLE public.iscrizione (
    id integer NOT NULL,
    anno smallint,
    dipartimento integer NOT NULL,
    ruolo integer NOT NULL,
    id_persona integer NOT NULL,
    fotourl character varying(150)
);


ALTER TABLE public.iscrizione OWNER TO giacomo;

--
-- TOC entry 225 (class 1259 OID 16559)
-- Name: iscrizione_dipartimento_seq; Type: SEQUENCE; Schema: public; Owner: giacomo
--

CREATE SEQUENCE public.iscrizione_dipartimento_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.iscrizione_dipartimento_seq OWNER TO giacomo;

--
-- TOC entry 5089 (class 0 OID 0)
-- Dependencies: 225
-- Name: iscrizione_dipartimento_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: giacomo
--

ALTER SEQUENCE public.iscrizione_dipartimento_seq OWNED BY public.iscrizione.dipartimento;


--
-- TOC entry 226 (class 1259 OID 16560)
-- Name: iscrizione_id_persona_seq; Type: SEQUENCE; Schema: public; Owner: giacomo
--

CREATE SEQUENCE public.iscrizione_id_persona_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.iscrizione_id_persona_seq OWNER TO giacomo;

--
-- TOC entry 5090 (class 0 OID 0)
-- Dependencies: 226
-- Name: iscrizione_id_persona_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: giacomo
--

ALTER SEQUENCE public.iscrizione_id_persona_seq OWNED BY public.iscrizione.id_persona;


--
-- TOC entry 227 (class 1259 OID 16561)
-- Name: iscrizione_id_seq; Type: SEQUENCE; Schema: public; Owner: giacomo
--

CREATE SEQUENCE public.iscrizione_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.iscrizione_id_seq OWNER TO giacomo;

--
-- TOC entry 5091 (class 0 OID 0)
-- Dependencies: 227
-- Name: iscrizione_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: giacomo
--

ALTER SEQUENCE public.iscrizione_id_seq OWNED BY public.iscrizione.id;


--
-- TOC entry 228 (class 1259 OID 16562)
-- Name: iscrizione_ruolo_seq; Type: SEQUENCE; Schema: public; Owner: giacomo
--

CREATE SEQUENCE public.iscrizione_ruolo_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.iscrizione_ruolo_seq OWNER TO giacomo;

--
-- TOC entry 5092 (class 0 OID 0)
-- Dependencies: 228
-- Name: iscrizione_ruolo_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: giacomo
--

ALTER SEQUENCE public.iscrizione_ruolo_seq OWNED BY public.iscrizione.ruolo;


--
-- TOC entry 229 (class 1259 OID 16563)
-- Name: news; Type: TABLE; Schema: public; Owner: giacomo
--

CREATE TABLE public.news (
    id integer NOT NULL,
    titolo character varying(50) NOT NULL,
    descrizione character varying(300),
    path_immagine character varying(150),
    data_rilascio date,
    categoria_id integer NOT NULL
);


ALTER TABLE public.news OWNER TO giacomo;

--
-- TOC entry 230 (class 1259 OID 16571)
-- Name: news_categoria_id_seq; Type: SEQUENCE; Schema: public; Owner: giacomo
--

CREATE SEQUENCE public.news_categoria_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.news_categoria_id_seq OWNER TO giacomo;

--
-- TOC entry 5093 (class 0 OID 0)
-- Dependencies: 230
-- Name: news_categoria_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: giacomo
--

ALTER SEQUENCE public.news_categoria_id_seq OWNED BY public.news.categoria_id;


--
-- TOC entry 231 (class 1259 OID 16572)
-- Name: news_id_seq; Type: SEQUENCE; Schema: public; Owner: giacomo
--

CREATE SEQUENCE public.news_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.news_id_seq OWNER TO giacomo;

--
-- TOC entry 5094 (class 0 OID 0)
-- Dependencies: 231
-- Name: news_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: giacomo
--

ALTER SEQUENCE public.news_id_seq OWNED BY public.news.id;


--
-- TOC entry 232 (class 1259 OID 16573)
-- Name: persona; Type: TABLE; Schema: public; Owner: giacomo
--

CREATE TABLE public.persona (
    id integer NOT NULL,
    nome character varying(20) NOT NULL,
    cognome character varying(20) NOT NULL,
    link character varying(200)
);


ALTER TABLE public.persona OWNER TO giacomo;

--
-- TOC entry 233 (class 1259 OID 16579)
-- Name: persona_id_seq; Type: SEQUENCE; Schema: public; Owner: giacomo
--

CREATE SEQUENCE public.persona_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.persona_id_seq OWNER TO giacomo;

--
-- TOC entry 5095 (class 0 OID 0)
-- Dependencies: 233
-- Name: persona_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: giacomo
--

ALTER SEQUENCE public.persona_id_seq OWNED BY public.persona.id;


--
-- TOC entry 234 (class 1259 OID 16580)
-- Name: ruolo; Type: TABLE; Schema: public; Owner: giacomo
--

CREATE TABLE public.ruolo (
    id integer NOT NULL,
    nome_ruolo character varying(30)
);


ALTER TABLE public.ruolo OWNER TO giacomo;

--
-- TOC entry 235 (class 1259 OID 16584)
-- Name: ruolo_id_seq; Type: SEQUENCE; Schema: public; Owner: giacomo
--

CREATE SEQUENCE public.ruolo_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.ruolo_id_seq OWNER TO giacomo;

--
-- TOC entry 5096 (class 0 OID 0)
-- Dependencies: 235
-- Name: ruolo_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: giacomo
--

ALTER SEQUENCE public.ruolo_id_seq OWNED BY public.ruolo.id;


--
-- TOC entry 4890 (class 2604 OID 16585)
-- Name: categoria id; Type: DEFAULT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.categoria ALTER COLUMN id SET DEFAULT nextval('public.category_id_seq'::regclass);


--
-- TOC entry 4891 (class 2604 OID 16586)
-- Name: dipartimento id; Type: DEFAULT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.dipartimento ALTER COLUMN id SET DEFAULT nextval('public.dipartimento_id_seq'::regclass);


--
-- TOC entry 4892 (class 2604 OID 16587)
-- Name: iscrizione id; Type: DEFAULT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.iscrizione ALTER COLUMN id SET DEFAULT nextval('public.iscrizione_id_seq'::regclass);


--
-- TOC entry 4893 (class 2604 OID 16588)
-- Name: iscrizione dipartimento; Type: DEFAULT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.iscrizione ALTER COLUMN dipartimento SET DEFAULT nextval('public.iscrizione_dipartimento_seq'::regclass);


--
-- TOC entry 4894 (class 2604 OID 16589)
-- Name: iscrizione ruolo; Type: DEFAULT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.iscrizione ALTER COLUMN ruolo SET DEFAULT nextval('public.iscrizione_ruolo_seq'::regclass);


--
-- TOC entry 4895 (class 2604 OID 16590)
-- Name: iscrizione id_persona; Type: DEFAULT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.iscrizione ALTER COLUMN id_persona SET DEFAULT nextval('public.iscrizione_id_persona_seq'::regclass);


--
-- TOC entry 4896 (class 2604 OID 16591)
-- Name: news id; Type: DEFAULT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.news ALTER COLUMN id SET DEFAULT nextval('public.news_id_seq'::regclass);


--
-- TOC entry 4897 (class 2604 OID 16592)
-- Name: news categoria_id; Type: DEFAULT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.news ALTER COLUMN categoria_id SET DEFAULT nextval('public.news_categoria_id_seq'::regclass);


--
-- TOC entry 4898 (class 2604 OID 16593)
-- Name: persona id; Type: DEFAULT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.persona ALTER COLUMN id SET DEFAULT nextval('public.persona_id_seq'::regclass);


--
-- TOC entry 4899 (class 2604 OID 16594)
-- Name: ruolo id; Type: DEFAULT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.ruolo ALTER COLUMN id SET DEFAULT nextval('public.ruolo_id_seq'::regclass);


--
-- TOC entry 5065 (class 0 OID 16530)
-- Dependencies: 219
-- Data for Name: _sqlx_migrations; Type: TABLE DATA; Schema: public; Owner: giacomo
--

COPY public._sqlx_migrations (version, description, installed_on, success, checksum, execution_time) FROM stdin;
\.


--
-- TOC entry 5066 (class 0 OID 16542)
-- Dependencies: 220
-- Data for Name: categoria; Type: TABLE DATA; Schema: public; Owner: giacomo
--

COPY public.categoria (id, nome) FROM stdin;
1	OFFICIAL
2	TECH
3	SPONSOR
\.


--
-- TOC entry 5068 (class 0 OID 16547)
-- Dependencies: 222
-- Data for Name: dipartimento; Type: TABLE DATA; Schema: public; Owner: giacomo
--

COPY public.dipartimento (id, nome) FROM stdin;
1	Avionics
3	Payload
4	Recovery system
5	Flight Dynamics
6	Marketing & Logistics
7	Avionics & Flight Dynamics
2	Propulsion & Structures & Manifacturing
8	Recovery
9	Propulsion, Structures and Manufacturing
10	Board
\.


--
-- TOC entry 5070 (class 0 OID 16552)
-- Dependencies: 224
-- Data for Name: iscrizione; Type: TABLE DATA; Schema: public; Owner: giacomo
--

COPY public.iscrizione (id, anno, dipartimento, ruolo, id_persona, fotourl) FROM stdin;
1	2024	1	1	1	Manuel Ferrazzini 2024.png
2	2024	2	1	2	Fabio Colzi 2024.png
3	2024	3	1	3	Marta Lacrimini 2024.png
4	2024	4	1	4	Manuel Gialli 2024.png
5	2024	5	1	5	Daniele De Simone 2024.png
6	2024	6	1	6	Federico Marconi 2024.png
7	2024	7	2	7	Niccolò Aurigi 2024.png
8	2024	7	2	8	Michele Betti 2024.png
9	2024	7	2	9	Emanuele Borghi 2024.png
10	2024	7	2	10	Andrea De Benedittis 2024.png
11	2024	7	2	11	Mattia Farina 2024.png
12	2024	7	2	12	Alessia Forcione 2024.png
13	2024	7	2	13	Divyanshi Mishra 2024.png
14	2024	7	2	14	Lorenzo Parisi 2024.png
15	2024	7	2	15	Matteo Raso 2024.png
16	2024	7	2	16	Denise Saccà 2024.png
17	2024	3	2	17	Konrad Barboutie 2024.png
18	2024	3	2	18	Paolo Sergio Castellani 2024.png
19	2024	3	2	19	Prabhjot Singh Maan 2024.png
20	2024	3	2	20	Antea Alderighi Sacchi 2024.png
21	2024	2	2	21	Michele Basteri 2024.png
22	2024	2	2	22	Mauro Furfari 2024.png
23	2024	2	2	23	Tommaso Garzia 2024.png
24	2024	2	2	24	Martina Lupi 2024.png
25	2024	2	2	25	Marco Mazzoni 2024.png
26	2024	2	2	26	Mario Nocilli 2024.png
27	2024	2	2	27	Matteo Trastulli 2024.png
28	2024	2	2	28	Daniel Tringali Ortisi 2024.png
29	2024	2	2	29	Emanuele Vezzi 2024.png
30	2024	4	2	30	Giezi Xitlali Chávez Serrano 2024.png
31	2024	4	2	31	Luca Dal Torrione 2024.png
32	2024	4	2	32	Matteo Gregorini 2024.png
33	2024	4	2	33	Leonardo Lupi 2024.png
34	2025	3	2	34	Tommaso Sorrentino 2025.jpg
35	2025	6	2	35	Thomas Vangelisti 2025.png
36	2025	3	2	36	Andrea Della Maggiora 2025.jpg
37	2025	3	2	37	Christian Kevin Alvarado Rimas 2025.jpeg
38	2025	6	2	38	matteo galeazzi 2025.jpg
39	2025	6	2	39	Francesco Durante 2025.jpg
40	2025	8	2	40	Riccardo Cappello 2025.jpg
41	2025	8	2	33	Leonardo Lupi 2025.jpg
42	2025	6	2	41	Giulia Macaluso 2025.png
43	2025	8	2	42	Giezi Xitlali Chavez Serrano 2025.jpg
44	2025	3	2	43	Emanuele Maria Sciortino 2025.jpg
45	2025	5	2	44	Martina Burgisi 2025.png
46	2025	9	2	45	Matheus Kruger Campaner 2025.jpg
47	2025	8	2	46	Diego Alejandro Valderrama Luna 2025.jpg
48	2025	10	2	22	Mauro Furfari 2025.jpg
49	2025	6	2	47	Riccardo Benedetti 2025.jpeg
50	2025	6	2	48	Bojidar Kuzmanov 2025.jpg
51	2025	10	2	49	Luca Dal Torrione  2025.jpg
52	2025	6	2	50	Riccardo  Ferrari  2025.jpg
53	2025	10	2	51	Gioele Gueli 2025.jpeg
54	2025	5	2	52	Matteo Di Sante 2025.png
55	2025	9	2	53	Leonardo  Stefanini  2025.jpg
56	2025	3	2	54	Iacopo  Pellegrinetti 2025.jpg
57	2025	10	2	55	Federico Manconi 2025.jpg
58	2025	5	2	56	Andrea Buggio 2025.png
59	2025	3	2	57	Dante  Sofia  2025.jpg
60	2025	9	2	58	Michele Basteri  2025.jpg
61	2025	9	2	59	Fabiano Nacci 2025.jpg
62	2025	1	2	60	Matteo Gabriele Causio 2025.jpg
63	2025	9	2	61	Mario  Nocilli  2025.jpg
64	2025	6	2	62	Lorenzo  Labarile 2025.jpg
65	2025	9	2	25	Marco Mazzoni 2025.jpeg
66	2025	6	2	63	Lorenzo Ligeri  2025.jpg
67	2025	10	2	64	andrea de benedittis 2025.jpg
68	2025	10	2	9	Emanuele Borghi 2025.jpg
69	2025	1	2	65	Victoria Ambler 2025.jpeg
70	2025	3	2	66	Konrad-Alexander Barboutie 2025.jpg
71	2025	5	2	67	Lorenzo  Menchini 2025.jpg
72	2025	1	2	68	David Moravac 2025.jpg
73	2025	8	2	69	Marika Tortolini 2025.jpg
74	2025	6	2	70	Lucio Merelli 2025.jpeg
75	2025	3	2	71	Pietro Bambagioni 2025.jpg
76	2025	6	2	72	Manuel Brian  De Michelis 2025.jpg
78	2025	9	2	27	Matteo Trastulli 2025.heic
82	2025	3	2	18	Paolo Sergio Castellani 2025.jpg
83	2025	8	2	75	Dasieva Muça 2025.jpg
84	2025	9	2	76	Sandu Patrascu 2025.jpg
85	2025	1	2	77	Alessandro Mauri 2025.jpg
88	2025	10	2	12	Alessia Forcione 2025.jpg
90	2025	9	2	29	Emanuele Vezzi 2025.jpeg
91	2025	9	2	79	Francesco Maradei 2025.jpeg
92	2025	6	2	80	Lorenzo Nepi 2025.jpeg
93	2025	9	2	81	Claudia Cobo 2025.jpg
94	2025	8	2	82	Emanuele Colombini 2025.jpg
95	2025	5	2	83	Teodoro Loiacono 2025.jpeg
96	2025	9	2	84	Francesco Stefanelli 2025.jpg
97	2025	5	2	85	Gabriele Patierno 2025.jpg
98	2025	9	2	86	Giulia Piombini 2025.jpeg
99	2025	9	2	87	Matilde Parigi 2025.jpeg
100	2025	9	2	88	Andrea Borgianni 2025.jpeg
101	2025	9	2	89	Samuele Nardinelli 2025.jpeg
102	2025	6	2	90	Zayd Aissaoui 2025.jpg
103	2025	9	2	91	Francesco  Becherini 2025.jpg
104	2025	9	2	92	Alessandro Centrella 2025.jpg
105	2025	9	2	93	Giulio De Lorenzo 2025.jpeg
106	2025	6	2	94	Nicola Alessandro Serraglini  2025.jpg
107	2025	9	2	95	Nicholas Leotta 2025.jpeg
108	2025	9	2	96	Francesco  Cicalè  2025.jpg
109	2025	6	2	97	Tommaso Bigagli 2025.jpeg
110	2025	1	2	98	Edoardo Maria  Di Cesare  2025.jpg
111	2025	6	2	99	Sara Del Gratta 2025.jpg
112	2025	8	2	100	Alberto Corbacella 2025.heic
113	2025	10	2	101	Federico Conti 2025.jpg
114	2025	6	2	102	Tommaso Corbani 2025.jpg
115	2025	8	2	103	Davide  Masciola 2025.jpg
116	2025	10	2	104	Mattia Drogo 2025.jpg
117	2025	10	2	105	Riccardo Scaletta 2025.png
118	2025	8	2	106	Samuel Le Rose 2025.jpg
119	2025	8	2	107	Giacomo Panci 2025.jpeg
120	2025	3	2	108	Nicolo Abruzzese 2025.jpg
121	2025	3	2	109	Samuele Bradke 2025.jpg
122	2025	5	2	110	Giacomo Consani 2025.jpg
123	2025	8	2	111	Eugenio Seva 2025.jpg
124	2025	6	2	112	Ginevra Pazzaglia  2025.jpg
125	2025	1	2	113	Adnaan Juma 2025.jpeg
126	2025	1	2	114	Valerio Leccese 2025.jpg
127	2025	6	2	115	Leonardo Mandoli 2025.jpg
77	2025	5	2	15	Matteo Raso 2025.jpeg
79	2025	9	2	23	Tommaso Garzia 2025.jpg
80	2025	8	2	73	Giorgio Charles Sorrentini 2025.png
81	2025	1	2	74	Luca Velardo 2025.png
86	2025	10	2	20	Antea Alderighi Sacchi 2025.jpeg
87	2025	6	2	78	Giorgia De Giorgi 2025.jpeg
89	2025	5	2	11	Mattia Farina 2025.jpg
\.


--
-- TOC entry 5075 (class 0 OID 16563)
-- Dependencies: 229
-- Data for Name: news; Type: TABLE DATA; Schema: public; Owner: giacomo
--

COPY public.news (id, titolo, descrizione, path_immagine, data_rilascio, categoria_id) FROM stdin;
1	penar	the peanits is good	/big_raga.webp	2026-01-01	1
2	mi piace la cioccolata calda	una tazza di cioccolata	/evil_car.gif	2026-01-02	2
\.


--
-- TOC entry 5078 (class 0 OID 16573)
-- Dependencies: 232
-- Data for Name: persona; Type: TABLE DATA; Schema: public; Owner: giacomo
--

COPY public.persona (id, nome, cognome, link) FROM stdin;
1	Manuel	Ferrazzini	https://www.linkedin.com/in/manuel-ferrazzani-776931100/
2	Fabio	Colzi	https://www.linkedin.com/in/fabio-colzi-6b9666296
3	Marta	Lacrimini	https://www.linkedin.com/in/marta-lacrimini-761b37302?utm_source=share&utm_campaign=share_via&utm_content=profile&utm_medium=ios_app
4	Manuel	Gialli	https://www.linkedin.com/in/manuel-gialli-866b46309?utm_source=share&utm_campaign=share_via&utm_content=profile&utm_medium=ios_app
5	Daniele	De Simone	https://www.linkedin.com/in/daniele-de-simone-38a28930b
6	Federico	Marconi	https://www.linkedin.com/in/federico-manconi/
7	Niccolò	Aurigi	https://www.linkedin.com/in/niccoloaurigi
8	Michele	Betti	https://www.linkedin.com/in/michele-betti-073a40296
9	Emanuele	Borghi	https://www.linkedin.com/in/emanueleborghi
10	Andrea	De Benedittis	https://www.linkedin.com/in/andrea-de-benedittis-a8792b342
11	Mattia	Farina	https://www.linkedin.com/in/mattia-farina-471208344
12	Alessia	Forcione	https://www.linkedin.com/in/alessia-forcione-56b817343
13	Divyanshi	Mishra	
14	Lorenzo	Parisi	
15	Matteo	Raso	https://www.linkedin.com/in/matteo-raso-18369130a
16	Denise	Saccà	https://www.linkedin.com/in/denise-sacc%C3%A0-235a03343
17	Konrad	Barboutie	https://www.linkedin.com/in/konrad-alexander-barboutie-ab746124b
18	Paolo Sergio	Castellani	https://www.linkedin.com/in/paolo-sergio-castellani-222a87307/
19	Prabhjot Singh	Maan	https://www.linkedin.com/in/prabhjotsinghmaan
20	Antea Alderighi	Sacchi	https://www.linkedin.com/in/antea-alderighi-sacchi-25398b31a
21	Michele	Basteri	https://www.linkedin.com/in/michele-basteri
22	Mauro	Furfari	https://www.linkedin.com/in/mauro-furfari/
23	Tommaso	Garzia	
24	Martina	Lupi	
25	Marco	Mazzoni	https://linkedin.com/in/marco-mazzoni-350400343
26	Mario	Nocilli	https://www.linkedin.com/in/mario-nocilli-088546236
27	Matteo	Trastulli	
28	Daniel	Tringali Ortisi	https://www.linkedin.com/in/daniel-tringali-7731871a7
29	Emanuele	Vezzi	https://www.linkedin.com/in/emanuele-vezzi-bba4a8166
30	Giezi Xitlali	Chávez Serrano	https://linkedin.com/in/xitlalichavez
31	Luca	Dal Torrione	https://www.linkedin.com/in/luca-dal-torrione-150b551a7
32	Matteo	Gregorini	
33	Leonardo	Lupi	https://www.linkedin.com/in/leonardo-lupi-a78b271bb
34	Tommaso	Sorrentino	
35	Thomas	Vangelisti	https://www.linkedin.com/feed/?trk=guest_homepage-basic_nav-header-signin
36	Andrea	Della Maggiora	
37	Christian Kevin	Alvarado Rimas	
38	matteo	galeazzi	https://www.linkedin.com/in/matteo-galeazzi1/
39	Francesco	Durante	
40	Riccardo	Cappello	https://www.linkedin.com/in/riccardo-cappello-7535b9249/
41	Giulia	Macaluso	https://www.linkedin.com/in/giulia-macaluso-9699a4362/
42	Giezi Xitlali	Chavez Serrano	https://www.linkedin.com/in/xitlalichavez/
43	Emanuele Maria	Sciortino	www.linkedin.com/in/emanuelesciortino
44	Martina	Burgisi	https://www.linkedin.com/in/martina-burgisi/
45	Matheus	Kruger Campaner	
46	Diego Alejandro	Valderrama Luna	www.linkedin.com/in/diego-v-09020a354
47	Riccardo	Benedetti	
48	Bojidar	Kuzmanov	
49	Luca	Dal Torrione 	https://www.linkedin.com/in/luca-dal-torrione-150b551a7?lipi=urn%3Ali%3Apage%3Ad_flagship3_profile_view_base_contact_details%3B24AIw%2Fy4RIy6BaRGuN9snQ%3D%3D
50	Riccardo 	Ferrari 	
51	Gioele	Gueli	https://www.linkedin.com/in/gioele-gueli-75b12539b 
52	Matteo	Di Sante	https://www.linkedin.com/in/matteo-di-sante-2a5a42228/
53	Leonardo 	Stefanini 	
54	Iacopo 	Pellegrinetti	
55	Federico	Manconi	https://www.linkedin.com/in/federico-manconi/
56	Andrea	Buggio	
57	Dante 	Sofia 	
58	Michele	Basteri 	https://www.linkedin.com/in/michele-basteri
59	Fabiano	Nacci	www.linkedin.com/in/fabiano-nacci-812b26170
60	Matteo Gabriele	Causio	https://www.linkedin.com/in/matteo-gabriele-causio-aa98a7377/
61	Mario 	Nocilli 	linkedin.com/in/marionocilli
62	Lorenzo 	Labarile	https://www.linkedin.com/profile/in/lorenzo-labarile-25243fgr1/
63	Lorenzo	Ligeri 	www.linkedin.com/in/lorenzo-ligeri-0133a6259 
64	andrea	de benedittis	Ce l’hai già queste robe mi la liberatoria è da modificare era una prova per sapere quanto la roba e modificami le 
65	Victoria	Ambler	https://www.linkedin.com/in/victoria-ambler-8b2b57398/
66	Konrad-Alexander	Barboutie	www.linkedin.com/in/konrad-alexander-barboutie-ab746124b
67	Lorenzo 	Menchini	https://www.linkedin.com/in/lorenzo-menchini
68	David	Moravac	www.linkedin.com/in/david-moravac-25424939a
69	Marika	Tortolini	https://www.linkedin.com/in/marika-tortolini-15b06a351?utm_source=share&utm_campaign=share_via&utm_content=profile&utm_medium=android_app
70	Lucio	Merelli	https://it.linkedin.com/in/lucio-merelli-928084234
71	Pietro	Bambagioni	https://www.linkedin.com/in/pietro-bambagioni-68b66738a/
72	Manuel Brian 	De Michelis	
73	Giorgio Charles	Sorrentini	https://www.linkedin.com/in/giorgio-charles-sorrentini-911a03235/
74	Luca	Velardo	https://www.linkedin.com/in/luca-velardo/
75	Dasieva	Muça	https://www.linkedin.com/in/dasieva-mu%C3%A7a-883a94192?utm_source=share&utm_campaign=share_via&utm_content=profile&utm_medium=ios_app
76	Sandu	Patrascu	https://www.linkedin.com/in/sandu-patrascu-27909b390/
77	Alessandro	Mauri	https://www.linkedin.com/in/alessandro-mauri-405185258/
78	Giorgia	De Giorgi	
79	Francesco	Maradei	https://www.linkedin.com/in/francesco-maradei-215b113a0?utm_source=share&utm_campaign=share_via&utm_content=profile&utm_medium=ios_app
80	Lorenzo	Nepi	https://www.linkedin.com/in/lorenzo-nepi-902a65387?utm_source=share&utm_campaign=share_via&utm_content=profile&utm_medium=ios_app 
81	Claudia	Cobo	https://www.linkedin.com/in/claudia-cobo-74a1a8397/
82	Emanuele	Colombini	
83	Teodoro	Loiacono	https://www.linkedin.com/in/teodoro-loiacono-8a3887300?utm_source=share&utm_campaign=share_via&utm_content=profile&utm_medium=ios_app
84	Francesco	Stefanelli	
85	Gabriele	Patierno	www.linkedin.com/in/gabrielepatierno
86	Giulia	Piombini	https://www.linkedin.com/in/giulia-piombini-894b57398?utm_source=share&utm_campaign=share_via&utm_content=profile&utm_medium=ios_app
87	Matilde	Parigi	www.linkedin.com/in/matilde-parigi-348a6a297
88	Andrea	Borgianni	www.linkedin.com/in/andrea-borgianni-3b91033a0
89	Samuele	Nardinelli	https://www.linkedin.com/in/samuele-nardinelli-1b6247386/
90	Zayd	Aissaoui	
91	Francesco 	Becherini	https://www.linkedin.com/in/francesco-becherini-94b1392b0?utm_source=share&utm_campaign=share_via&utm_content=profile&utm_medium=android_app
92	Alessandro	Centrella	
93	Giulio	De Lorenzo	
94	Nicola Alessandro	Serraglini 	https://www.linkedin.com/in/nicola-serraglini-239768103?utm_source=share&utm_campaign=share_via&utm_content=profile&utm_medium=android_app
95	Nicholas	Leotta	non ho un profilo linkedin
96	Francesco 	Cicalè 	https://www.linkedin.com/in/francesco-cical%C3%A8-4aa54a3a0
97	Tommaso	Bigagli	https://www.linkedin.com/in/tommaso-bigagli-03728a328?utm_source=share&utm_campaign=share_via&utm_content=profile&utm_medium=ios_app
98	Edoardo Maria 	Di Cesare 	www.linkedin.com/in/edoardo-maria-di-cesare-5492743a0
99	Sara	Del Gratta	
100	Alberto	Corbacella	
101	Federico	Conti	www.linkedin.com/in/federico-conti-a6a777334
102	Tommaso	Corbani	
103	Davide 	Masciola	
104	Mattia	Drogo	https://www.linkedin.com/in/mattia-drogo-911259293/
105	Riccardo	Scaletta	https://www.linkedin.com/in/riccardo-scaletta-882b973b0/
106	Samuel	Le Rose	
107	Giacomo	Panci	
108	Nicolo	Abruzzese	www.linkedin.com/in/nicoló-abruzzese-302b06316
109	Samuele	Bradke	https://www.linkedin.com/in/samuele-bradke/
110	Giacomo	Consani	www.linkedin.com/in/giacomo-consani-82a33a2b2
111	Eugenio	Seva	
112	Ginevra	Pazzaglia 	
113	Adnaan	Juma	https://www.linkedin.com/in/adnaanibnjuma/
114	Valerio	Leccese	www.linkedin.com/in/valerio-leccese-044954396
115	Leonardo	Mandoli	https://www.linkedin.com/in/leonardomandoli
\.


--
-- TOC entry 5080 (class 0 OID 16580)
-- Dependencies: 234
-- Data for Name: ruolo; Type: TABLE DATA; Schema: public; Owner: giacomo
--

COPY public.ruolo (id, nome_ruolo) FROM stdin;
1	chief
2	user
\.


--
-- TOC entry 5097 (class 0 OID 0)
-- Dependencies: 221
-- Name: category_id_seq; Type: SEQUENCE SET; Schema: public; Owner: giacomo
--

SELECT pg_catalog.setval('public.category_id_seq', 3, true);


--
-- TOC entry 5098 (class 0 OID 0)
-- Dependencies: 223
-- Name: dipartimento_id_seq; Type: SEQUENCE SET; Schema: public; Owner: giacomo
--

SELECT pg_catalog.setval('public.dipartimento_id_seq', 10, true);


--
-- TOC entry 5099 (class 0 OID 0)
-- Dependencies: 225
-- Name: iscrizione_dipartimento_seq; Type: SEQUENCE SET; Schema: public; Owner: giacomo
--

SELECT pg_catalog.setval('public.iscrizione_dipartimento_seq', 1, false);


--
-- TOC entry 5100 (class 0 OID 0)
-- Dependencies: 226
-- Name: iscrizione_id_persona_seq; Type: SEQUENCE SET; Schema: public; Owner: giacomo
--

SELECT pg_catalog.setval('public.iscrizione_id_persona_seq', 1, false);


--
-- TOC entry 5101 (class 0 OID 0)
-- Dependencies: 227
-- Name: iscrizione_id_seq; Type: SEQUENCE SET; Schema: public; Owner: giacomo
--

SELECT pg_catalog.setval('public.iscrizione_id_seq', 127, true);


--
-- TOC entry 5102 (class 0 OID 0)
-- Dependencies: 228
-- Name: iscrizione_ruolo_seq; Type: SEQUENCE SET; Schema: public; Owner: giacomo
--

SELECT pg_catalog.setval('public.iscrizione_ruolo_seq', 1, false);


--
-- TOC entry 5103 (class 0 OID 0)
-- Dependencies: 230
-- Name: news_categoria_id_seq; Type: SEQUENCE SET; Schema: public; Owner: giacomo
--

SELECT pg_catalog.setval('public.news_categoria_id_seq', 1, false);


--
-- TOC entry 5104 (class 0 OID 0)
-- Dependencies: 231
-- Name: news_id_seq; Type: SEQUENCE SET; Schema: public; Owner: giacomo
--

SELECT pg_catalog.setval('public.news_id_seq', 2, true);


--
-- TOC entry 5105 (class 0 OID 0)
-- Dependencies: 233
-- Name: persona_id_seq; Type: SEQUENCE SET; Schema: public; Owner: giacomo
--

SELECT pg_catalog.setval('public.persona_id_seq', 115, true);


--
-- TOC entry 5106 (class 0 OID 0)
-- Dependencies: 235
-- Name: ruolo_id_seq; Type: SEQUENCE SET; Schema: public; Owner: giacomo
--

SELECT pg_catalog.setval('public.ruolo_id_seq', 2, true);


--
-- TOC entry 4901 (class 2606 OID 16596)
-- Name: _sqlx_migrations _sqlx_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public._sqlx_migrations
    ADD CONSTRAINT _sqlx_migrations_pkey PRIMARY KEY (version);


--
-- TOC entry 4903 (class 2606 OID 16598)
-- Name: categoria category_pkey; Type: CONSTRAINT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.categoria
    ADD CONSTRAINT category_pkey PRIMARY KEY (id);


--
-- TOC entry 4905 (class 2606 OID 16600)
-- Name: dipartimento dipartimento_pkey; Type: CONSTRAINT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.dipartimento
    ADD CONSTRAINT dipartimento_pkey PRIMARY KEY (id);


--
-- TOC entry 4907 (class 2606 OID 16602)
-- Name: iscrizione iscrizione_pkey; Type: CONSTRAINT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.iscrizione
    ADD CONSTRAINT iscrizione_pkey PRIMARY KEY (id);


--
-- TOC entry 4909 (class 2606 OID 16604)
-- Name: news news_pkey; Type: CONSTRAINT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.news
    ADD CONSTRAINT news_pkey PRIMARY KEY (id);


--
-- TOC entry 4911 (class 2606 OID 16606)
-- Name: persona persona_pkey; Type: CONSTRAINT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.persona
    ADD CONSTRAINT persona_pkey PRIMARY KEY (id);


--
-- TOC entry 4913 (class 2606 OID 16608)
-- Name: ruolo ruolo_pkey; Type: CONSTRAINT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.ruolo
    ADD CONSTRAINT ruolo_pkey PRIMARY KEY (id);


--
-- TOC entry 4914 (class 2606 OID 16609)
-- Name: iscrizione iscrizione_dipartimento_fkey; Type: FK CONSTRAINT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.iscrizione
    ADD CONSTRAINT iscrizione_dipartimento_fkey FOREIGN KEY (dipartimento) REFERENCES public.dipartimento(id);


--
-- TOC entry 4915 (class 2606 OID 16614)
-- Name: iscrizione iscrizione_id_persona_fkey; Type: FK CONSTRAINT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.iscrizione
    ADD CONSTRAINT iscrizione_id_persona_fkey FOREIGN KEY (id_persona) REFERENCES public.persona(id);


--
-- TOC entry 4916 (class 2606 OID 16619)
-- Name: iscrizione iscrizione_ruolo_fkey; Type: FK CONSTRAINT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.iscrizione
    ADD CONSTRAINT iscrizione_ruolo_fkey FOREIGN KEY (ruolo) REFERENCES public.ruolo(id);


--
-- TOC entry 4917 (class 2606 OID 16624)
-- Name: news news_categoria_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: giacomo
--

ALTER TABLE ONLY public.news
    ADD CONSTRAINT news_categoria_id_fkey FOREIGN KEY (categoria_id) REFERENCES public.categoria(id);


-- Completed on 2026-03-23 16:54:32

--
-- PostgreSQL database dump complete
--

\unrestrict zd3QxTTP8fcpxDce1fc0pvbRCyWleZEKtGXgWTXd92wM8NX0eMTMJg92NPi0GVX

