.[] | {
  "@context": {
    "know": "https://know.dev/",
    "xsd": "http://www.w3.org/2001/XMLSchema#",
    "icon": {
      "@id": "know:icon",
      "@type": "@id",
    },
    "image": {
      "@id": "know:image",
      "@type": "@id",
    },
    "items": {
      "@id": "know:items",
      "@type": "know:SearchResult",
      "@container": "@list",
    },
    "link": {
      "@id": "know:link",
      "@type": "@id",
    },
    "position": {
      "@id": "know:position",
      "@type": "xsd:integer",
    },
    "summary": {
      "@id": "know:summary",
      "@language": "en",
    },
    "title": {
      "@id": "know:title",
      "@language": "en",
    },
  },
  "@id": .url,
  "@type": "know:SearchResults",
  "title": .searchQuery.term,
  "summary": null,
  "image": null,
  "items": [
    .organicResults[] | {
      "@type": "know:SearchResult",
      "position": .position,
      "title": .title,
      "summary": .description,
      "link": .url,
      "icon": null,
    }
  ],
}
