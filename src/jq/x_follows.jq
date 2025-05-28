{
  "@context": {
    "know": "https://know.dev/",
    "xsd": "http://www.w3.org/2001/XMLSchema#",
    "rdfs": "http://www.w3.org/2000/01/rdf-schema#",
    "count": {
      "@id": "know:count",
      "@type": "xsd:integer",
    },
    "followees": {
      "@id": "know:followees",
      "@type": "know:Collection",
    },
    "followers": {
      "@id": "know:followers",
      "@type": "know:Collection",
    },
    "id": {
      "@id": "know:id",
      "@type": "xsd:string",
    },
    "image": {
      "@id": "know:image",
      "@type": "@id",
    },
    "items": {
      "@id": "rdfs:member",
      "@type": "know:UserAccount",
      "@container": "@set",
    },
    "link": {
      "@id": "know:link",
      "@type": "@id",
    },
    "name": {
      "@id": "know:name",
      "@type": "xsd:string",
    },
    "posts": {
      "@id": "know:posts",
      "@type": "know:Collection",
    },
    "subscriptions": {
      "@id": "know:subscriptions",
      "@type": "know:Collection",
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
  "@id": ("https://x.com/" + .[0].target_username),
  "@type": ["know:UserProfile", "know:UserAccount"],
  "id": .[0].target_username,
  "followees": {
    "items": [
      .[] | select(.type == "following") | {
        "@id": ("https://x.com/" + .screen_name),
        "@type": ["know:UserProfile", "know:UserAccount"],
        "id": null,
        "name": .screen_name,
        "title": .name,
        "summary": .description,
        "image": .profile_image_url_https,
        "link": (try .entities.url.urls[0].expanded_url catch null),
      }
    ]
  },
  "followers": {
    "items": [
      .[] | select(.type == "follower") | {
        "@id": ("https://x.com/" + .screen_name),
        "@type": ["know:UserProfile", "know:UserAccount"],
        "id": null,
        "name": .screen_name,
        "title": .name,
        "summary": .description,
        "image": .profile_image_url_https,
        "link": (try .entities.url.urls[0].expanded_url catch null),
      }
    ]
  },
}
