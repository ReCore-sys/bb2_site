package site

import (
	"fmt"
	"net/http"

	"github.com/ReCore-sys/bottombot2/libs/config"
	mongo "github.com/ReCore-sys/bottombot2/libs/database"
	"github.com/ReCore-sys/bottombot2/libs/logging"
	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
)

var CFG = config.Config()

func Entry() {
	r := gin.Default()
	r.Use(cors.Default())
	r.NoRoute(gin.WrapH(http.FileServer(gin.Dir("site/dist", false))))
	api := r.Group("/api")
	/**========================================================================
	 *                           API points
	 *========================================================================**/

	api.GET("/ping/", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"message": "pong",
		})
	})

	api.GET("/users/", func(c *gin.Context) {
		up := mongo.IsUp()
		if !up {
			c.AbortWithStatusJSON(500, gin.H{
				"error":  "Database connection failing",
				"source": "api/users: check database",
			})
			return
		}
		db, err := mongo.OpenSession(CFG.Server, CFG.DBPort, CFG.Collection)
		if err != nil {
			logging.Log(err)
			c.AbortWithStatusJSON(500, gin.H{
				"error":  err.Error(),
				"source": "api/users: open db",
			})
			return
		}
		defer db.Close()
		c.JSON(200, gin.H{
			"users": db.GetAll(),
		})
	})

	api.GET("/config/", func(c *gin.Context) {
		newconfig := config.Config()
		newconfig.Token = "***"
		newconfig.DBPort = 0
		c.JSON(200, newconfig)
	})

	api.POST("/feedback/", func(c *gin.Context) {
		jsonData, err := c.GetRawData()
		if err != nil {
			logging.Log(err)
			c.AbortWithStatusJSON(500, gin.H{
				"error":  err.Error(),
				"source": "api/feedback: read body",
			})
			return
		}

		fmt.Printf("%+v\n", string(jsonData))
	})

	err := r.Run(":80") // listen and serve on 0.0.0.0:8080
	if err != nil {
		logging.Log(err)
	}
}
